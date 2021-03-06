#![no_main]
#![no_std]
// #![deny(warnings)]
// #![feature(alloc_error_handler)]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc_cortex_m::CortexMHeap;
// use core::alloc::Layout;
use defmt_rtt as _; // global logger
use panic_probe as _;
use stm32h7xx_hal as _; // memory layout

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
  cortex_m::asm::udf()
}

use core::sync::atomic::{AtomicUsize, Ordering};
static COUNT: AtomicUsize = AtomicUsize::new(0);
defmt::timestamp!("{=usize}", {
  // NOTE(no-CAS) `timestamps` runs with interrupts disabled
  let n = COUNT.load(Ordering::Relaxed);
  COUNT.store(n + 1, Ordering::Relaxed);
  n
});

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

// #[alloc_error_handler]
// fn oom(_: Layout) -> ! {
//   loop {}
// }

mod framebuffer;
mod keypad;
mod types;
mod update;
mod util;
mod view;

#[rtic::app(device = stm32h7xx_hal::stm32, peripherals = true, dispatchers = [USART1, USART2, EXTI0])]
mod app {
  use crate::framebuffer::Framebuffer;
  use crate::keypad::{self, EventBufferUtil, KeypadRead};
  use crate::types::{BacklightLED, Cmd, Display, KeyType, KeypadMode, Model, Msg, Screen};
  use crate::update::update;
  use crate::view::view;
  use alloc::vec::Vec;
  use asm_delay::{bitrate, AsmDelay};
  use display_interface_spi::SPIInterface;
  use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
  use embedded_hal::spi::{Mode, Phase, Polarity};
  use embedded_hal::{digital::v2::OutputPin, prelude::*};
  use embedded_sdmmc::{Controller, File, TimeSource, Timestamp, VolumeIdx};
  use fatfs::{FileSystem, FsOptions};
  use keypad2::Keypad;
  use st7789::{Orientation, TearingEffect, ST7789};
  use stm32h7xx_hal::device::SDMMC1;
  use stm32h7xx_hal::sdmmc::{Sdmmc, SdmmcBlockDevice};
  use stm32h7xx_hal::{prelude::*, rcc};
  use systick_monotonic::*;

  pub struct SdClock;

  impl TimeSource for SdClock {
    fn get_timestamp(&self) -> Timestamp {
      Timestamp {
        year_since_1970: 0,
        zero_indexed_month: 0,
        zero_indexed_day: 0,
        hours: 0,
        minutes: 0,
        seconds: 0,
      }
    }
  }

  #[monotonic(binds = SysTick, default = true)]
  type MyMono = Systick<480>; // 480 Hz / 10 ms granularity

  #[shared]
  struct Shared {
    should_render: bool,
    state: Model,
    keypad: keypad::Keys,
  }

  #[local]
  struct Local {
    delay: AsmDelay,
    display: Display,
    backlight: BacklightLED,
    event_buffer: Option<keypad::EventBuffer>,
    framebuffer: Framebuffer,
    // sd: Sdmmc<SDMMC1>,
    sd_fatfs: Option<Controller<SdmmcBlockDevice<Sdmmc<SDMMC1>>, SdClock>>,
  }

  #[init]
  fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
    unsafe {
      super::ALLOCATOR.init(cortex_m_rt::heap_start() as usize, 512);
    };

    defmt::info!("INIT");
    let pwrcfg = ctx.device.PWR.constrain().vos0(&ctx.device.SYSCFG).freeze();

    // Set up the system clock.
    let ccdr = ctx
      .device
      .RCC
      .constrain()
      .sys_ck(480.mhz())
      .pll1_strategy(rcc::PllConfigStrategy::Iterative)
      .pll1_q_ck(100.mhz())
      .pll2_strategy(rcc::PllConfigStrategy::Iterative)
      .pll3_strategy(rcc::PllConfigStrategy::Iterative)
      .freeze(pwrcfg, &ctx.device.SYSCFG);

    let mono = Systick::<480>::new(ctx.core.SYST, 480_000_000);

    let gpioa = ctx.device.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpiob = ctx.device.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioc = ctx.device.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = ctx.device.GPIOD.split(ccdr.peripheral.GPIOD);
    let gpioe = ctx.device.GPIOE.split(ccdr.peripheral.GPIOE);

    let mut delay = AsmDelay::new(bitrate::MegaHertz(480));

    let (mut display, mut backlight) = {
      let sck1 = gpioa.pa5.into_alternate_af5();
      let miso1 = gpioa.pa6.into_alternate_af5();
      let mosi1 = gpioa.pa7.into_alternate_af5();
      let spi1 = ctx.device.SPI1.spi(
        (sck1, miso1, mosi1),
        Mode {
          polarity: Polarity::IdleLow,
          phase: Phase::CaptureOnFirstTransition,
        },
        30.mhz(), // 32 mhz is the highest freq that works based on testing
        ccdr.peripheral.SPI1,
        &ccdr.clocks,
      );

      let chip_select = gpioa.pa3.into_push_pull_output(); // chip select
      let data_control = gpioa.pa0.into_push_pull_output(); // data control
      let display_interface = SPIInterface::new(spi1, data_control, chip_select);

      let lcd_reset = gpioa.pa2.into_push_pull_output();
      let backlight = gpioa.pa1.into_push_pull_output();

      let display = ST7789::new(display_interface, lcd_reset, 320, 240);

      (display, backlight)
    };

    // a certain amount of delay is necessary so the display responds on first boot
    delay.delay_ms(250u16);

    // Initialise the display and clear the screen
    display.init(&mut delay).unwrap();

    display.set_orientation(Orientation::Landscape).unwrap();

    delay.delay_ms(250u16);

    display.set_tearing_effect(TearingEffect::Off).unwrap();

    display.clear(Rgb565::BLACK).unwrap();

    backlight.set_high().unwrap();

    let keypad = {
      let cols = (
        gpioe.pe11.into_open_drain_output(),
        gpioe.pe13.into_open_drain_output(),
        gpioe.pe15.into_open_drain_output(),
      );
      let rows = (
        gpioc.pc5.into_pull_up_input(),
        gpiob.pb1.into_pull_up_input(),
        gpioe.pe7.into_pull_up_input(),
        gpioe.pe9.into_pull_up_input(),
      );

      Keypad::new(rows, cols)
    };

    let sd_fatfs: Option<Controller<SdmmcBlockDevice<Sdmmc<SDMMC1>>, SdClock>> = {
      // SDMMC1 pins
      let clk = gpioc.pc12.into_alternate_af12();
      let cmd = gpiod.pd2.into_alternate_af12();
      let d0 = gpioc.pc8.into_alternate_af12();
      let d1 = gpioc.pc9.into_alternate_af12();
      let d2 = gpioc.pc10.into_alternate_af12();
      let d3 = gpioc.pc11.into_alternate_af12();

      let mut sd: Sdmmc<SDMMC1> = ctx.device.SDMMC1.sdmmc(
        (clk, cmd, d0, d1, d2, d3),
        ccdr.peripheral.SDMMC1,
        &ccdr.clocks,
      );

      // let img_file = File::open("fat.img").unwrap();
      // let fs = fatfs::FileSystem::new(img_file, fatfs::FsOptions::new()).unwrap();

      // On most development boards this can be increased up to 50MHz. We choose a
      // lower frequency here so that it should work even with flying leads
      // connected to a SD card breakout.
      match sd.init_card(2.mhz()) {
        Ok(_) => {
          let size = sd.card().unwrap().size();
          defmt::info!("SD Size: {}", size);

          let sd_fatfs = Controller::new(sd.sdmmc_block_device(), SdClock);

          Some(sd_fatfs)
        }
        Err(err) => {
          defmt::info!("{:?}", defmt::Debug2Format(&err));
          None
        }
      }
    };

    defmt::info!("INIT DONE");

    (
      Shared {
        should_render: true,
        state: Model::default(),
        keypad,
      },
      Local {
        display,
        backlight,
        delay,
        event_buffer: None,
        framebuffer: Framebuffer::new(),
        sd_fatfs,
      },
      init::Monotonics(mono),
    )
  }

  #[idle]
  fn idle(_ctx: idle::Context) -> ! {
    defmt::info!("start event loop");
    update_task::spawn(Msg::Navigate(Screen::Splash)).unwrap();
    event_loop_task::spawn().unwrap();

    loop {
      // loop necessary for RTT to work
      cortex_m::asm::nop();
    }
  }

  #[task(priority = 3, shared = [state, should_render], local = [sd_fatfs])]
  fn update_task(ctx: update_task::Context, msg: Msg) {
    let update_task::SharedResources {
      should_render,
      state,
    } = ctx.shared;
    let local = ctx.local;
    let update_task::LocalResources { sd_fatfs } = local;

    (should_render, state).lock(|should_render, state| {
      let cmd = update(state, msg);
      match cmd {
        Cmd::UpdateAfter(time_ms, msg) => {
          update_task::spawn_after(time_ms.millis(), msg).unwrap();
        }
        Cmd::InitSD => {
          if let Some(sd) = sd_fatfs {
            let sd: &mut Controller<SdmmcBlockDevice<Sdmmc<SDMMC1>>, SdClock> = sd;
            let mut volume = sd.get_volume(VolumeIdx(0)).unwrap();
            let root_dir = sd.open_root_dir(&volume).unwrap();


            // let file = sd
            //   .open_file_in_dir(
            //     &mut volume,
            //     &root_dir,
            //     "BASE64~1.PSB",
            //     embedded_sdmmc::Mode::ReadOnly,
            //   )
            //   .unwrap();

            sd.iterate_dir(&volume, &root_dir, |entry| {
              defmt::info!("{:?}", defmt::Debug2Format(&entry));
            })
            .unwrap();

            sd.close_dir(&volume, root_dir);
          }
        }
        Cmd::None => {}
      };

      *should_render = true;
    });
  }

  #[task(priority = 2, local = [event_buffer, delay], shared = [keypad, state])]
  fn keypad_task(ctx: keypad_task::Context) -> () {
    let keypad_task::LocalResources {
      event_buffer,
      delay,
    } = ctx.local;
    let keypad_task::SharedResources {
      mut keypad,
      mut state,
    } = ctx.shared;

    let now = monotonics::now();

    if event_buffer.is_none() {
      // set default value
      *event_buffer = Some([keypad::ButtonEvent { button: None, now }; 8]);
    }

    let mut key: Option<keypad::Key> = None;

    if let Some(event_buffer) = event_buffer {
      keypad.lock(|keypad| {
        let read_button = keypad.read(delay);

        let last_button = event_buffer[0].button;
        let both_none = last_button.is_none() && read_button.is_none();
        if !both_none {
          if last_button.is_some() && read_button.is_none() {
            // key up
            event_buffer.unshift(keypad::ButtonEvent { button: None, now });
          }
          if last_button.is_none() && read_button.is_some() {
            // key down
            event_buffer.unshift(keypad::ButtonEvent {
              button: read_button,
              now,
            });
          }

          if last_button.is_some() && read_button.is_some() && last_button != read_button {
            // key down, but different button
            event_buffer.unshift(keypad::ButtonEvent {
              button: read_button,
              now,
            });
          }
        }
      });

      if event_buffer[0].is_none() {
        // wait until key up

        state.lock(|state| {
          match state.keypad_mode {
            KeypadMode::Navigation => {
              if let Some(button) = event_buffer[1].button {
                let direction = match button {
                  keypad::Button::Two => Some(keypad::NavigationKey::Up),
                  keypad::Button::Four => Some(keypad::NavigationKey::Left),
                  keypad::Button::Six => Some(keypad::NavigationKey::Right),
                  keypad::Button::Eight => Some(keypad::NavigationKey::Down),

                  keypad::Button::Back => Some(keypad::NavigationKey::Back),
                  keypad::Button::Forward => Some(keypad::NavigationKey::Forward),
                  _ => None,
                };
                // using the keypad in navigation mode will not clear the event_buffer
                if let Some(direction) = direction {
                  event_buffer.unshift(keypad::ButtonEvent { button: None, now });
                  event_buffer.unshift(keypad::ButtonEvent { button: None, now });
                  update_task::spawn(Msg::KeyUp(KeyType::Navigation(direction))).unwrap();
                }
              }
            }
            KeypadMode::Number => {
              if let Some(button) = event_buffer[1].button {
                let number = match button {
                  keypad::Button::Zero => keypad::NumberKey::Zero,
                  keypad::Button::One => keypad::NumberKey::One,
                  keypad::Button::Two => keypad::NumberKey::Two,
                  keypad::Button::Three => keypad::NumberKey::Three,
                  keypad::Button::Four => keypad::NumberKey::Four,
                  keypad::Button::Five => keypad::NumberKey::Five,
                  keypad::Button::Six => keypad::NumberKey::Six,
                  keypad::Button::Seven => keypad::NumberKey::Seven,
                  keypad::Button::Eight => keypad::NumberKey::Eight,
                  keypad::Button::Nine => keypad::NumberKey::Nine,
                  keypad::Button::Back => keypad::NumberKey::Back,
                  keypad::Button::Forward => keypad::NumberKey::Forward,
                };
                // using the keypad in number mode will not clear the event_buffer
                event_buffer.unshift(keypad::ButtonEvent { button: None, now });
                event_buffer.unshift(keypad::ButtonEvent { button: None, now });
                update_task::spawn(Msg::KeyUp(KeyType::Number(number))).unwrap();
              }
            }
            KeypadMode::Text => {
              if keypad::check_timespan_ms(&event_buffer[0].now, &now, 200) {
                // enough time has passed to process an event

                if let Some(last_button) = event_buffer[1].button {
                  if (last_button == keypad::Button::Seven || last_button == keypad::Button::Nine)
                    && event_buffer.check_for_quad()
                  {
                    // only buttons that can be pressed 4 times are 7 an 9
                    // last 4 are the same
                    key = last_button.to_key(4);
                  } else if event_buffer.check_for_triple() {
                    // last 3 are the same
                    key = last_button.to_key(3);
                  } else if event_buffer.check_for_double() {
                    // last 2 are the same
                    key = last_button.to_key(2);
                  } else if event_buffer.check_for_single() {
                    // just one
                    key = last_button.to_key(1);
                  }
                }
              }
            }
          }
        });
      }
    }

    // there is a key to be processed
    if let Some(key) = key {
      // wipe the buffer
      *event_buffer = None;
      // send event
      update_task::spawn(Msg::KeyUp(KeyType::Text(key))).unwrap();
    }
  }

  #[task(priority = 2, shared = [state, should_render], local = [display, backlight, framebuffer])]
  fn render_task(ctx: render_task::Context) {
    let render_task::SharedResources {
      should_render,
      state,
    } = ctx.shared;
    let render_task::LocalResources {
      display,
      backlight,
      framebuffer,
    } = ctx.local;

    (should_render, state).lock(|should_render, state| {
      if *should_render {
        // backlight.set_low().unwrap();

        view(framebuffer, &state).unwrap();
        framebuffer.draw(display).unwrap();

        // view(display, &state).unwrap();

        // backlight.set_high().unwrap();

        *should_render = false;
      }
    });
  }

  #[task(priority = 1)]
  fn event_loop_task(_ctx: event_loop_task::Context) -> () {
    keypad_task::spawn().unwrap();
    // update_task runs after called by keypad_task or update_task
    // update_task has the highest priority
    // keypad_task and render_task have the same priority because they're only
    // called by event_loop_task
    render_task::spawn().unwrap();

    event_loop_task::spawn_after(15.millis()).unwrap();
  }
}
