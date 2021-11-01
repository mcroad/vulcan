#![no_main]
#![no_std]
#![deny(unused_imports)]
#![deny(unsafe_code)]
#![deny(warnings)]

use defmt_rtt as _; // global logger
use panic_probe as _;
use stm32f4xx_hal as _; // memory layout

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

mod home;
mod splash;
mod types;
mod update;
mod util;
mod view;

#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, dispatchers = [USART1, USART2, EXTI0])]
mod app {
  use crate::types::{BacklightLED, Display, KeypadMode, Keys, Msg, Screen, State};
  use crate::update::update;
  use crate::util::{Button, Key, KeypadRead};
  use crate::view::view;
  use asm_delay::bitrate;
  use asm_delay::AsmDelay;
  use display_interface_spi::SPIInterface;
  use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
  use embedded_hal::prelude::*;
  use embedded_hal::spi::{Mode, Phase, Polarity};
  use heapless::String;
  use keypad2::Keypad;
  use rtic::time::duration::*;
  use st7789::{Orientation, ST7789};
  use stm32f4xx_hal::{prelude::*, spi::Spi, time::MegaHertz};
  use systick_monotonic::Systick;

  #[monotonic(binds = SysTick, default = true)]
  type MyMono = Systick<100>; // 100 Hz / 10 ms granularity

  #[shared]
  struct Shared {
    should_render: bool,
    state: State,
    keypad: Keys,
  }

  #[local]
  struct Local {
    delay: AsmDelay,
    delay2: AsmDelay,
    display: Display,
    backlight: BacklightLED,
    last_keys: [Option<Button>; 5],
  }

  #[init]
  fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
    defmt::info!("INIT");

    ctx.core.DCB.enable_trace();
    ctx.core.DWT.enable_cycle_counter();

    // Set up the system clock.
    let rcc = ctx.device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(100.mhz()).freeze();

    let mono = Systick::<100>::new(ctx.core.SYST, 100_000_000);

    let gpioa = ctx.device.GPIOA.split();
    let gpiob = ctx.device.GPIOB.split();

    let mut delay = AsmDelay::new(bitrate::MegaHertz(100));

    let mut display = {
      let sck2 = gpiob.pb10.into_alternate_af5();
      let miso2 = gpiob.pb14.into_alternate_af5();
      let mosi2 = gpiob.pb15.into_alternate_af5();
      let spi2 = Spi::spi2(
        ctx.device.SPI2,
        (sck2, miso2, mosi2),
        Mode {
          polarity: Polarity::IdleLow,
          phase: Phase::CaptureOnFirstTransition,
        },
        MegaHertz(8).into(),
        clocks,
      );

      let chip_select = gpioa.pa4.into_push_pull_output(); // chip select
      let data_control = gpioa.pa1.into_push_pull_output(); // data control
      let display_interface = SPIInterface::new(spi2, data_control, chip_select);

      let lcd_reset = gpioa.pa3.into_push_pull_output();

      ST7789::new(display_interface, lcd_reset, 240, 240)
    };

    // a certain amount of delay is necessary so the display responds on first boot
    delay.delay_ms(250u16);

    // Initialise the display and clear the screen
    display.init(&mut delay).unwrap();

    display.set_orientation(Orientation::Landscape).unwrap();

    delay.delay_ms(250u16);

    display.clear(Rgb565::BLACK).unwrap();

    // backlight control
    let mut backlight = gpioa.pa2.into_push_pull_output();
    backlight.set_high().unwrap();

    let keypad = {
      let cols = (
        gpioa.pa8.into_open_drain_output(),
        gpioa.pa9.into_open_drain_output(),
        gpioa.pa10.into_open_drain_output(),
      );
      let rows = (
        gpioa.pa11.into_pull_up_input(),
        gpioa.pa12.into_pull_up_input(),
        gpioa.pa15.into_pull_up_input(),
        gpiob.pb3.into_pull_up_input(),
      );

      Keypad::new(rows, cols)
    };

    keypad_task::spawn().unwrap();

    defmt::info!("INIT DONE");

    (
      Shared {
        should_render: true,
        state: State {
          screen: Screen::Splash,
          msg: String::from("home screen"),
          keypad_mode: KeypadMode::Text,
        },
        keypad,
      },
      Local {
        display,
        backlight,
        delay,
        delay2: AsmDelay::new(bitrate::MegaHertz(100)),
        last_keys: [None; 5],
      },
      init::Monotonics(mono),
    )
  }

  #[idle(local = [delay])]
  fn idle(ctx: idle::Context) -> ! {
    let idle::LocalResources { delay } = ctx.local;

    update_task::spawn(Msg::Navigate(Screen::Splash)).unwrap();
    render_task::spawn().unwrap();

    delay.delay_ms(2000u16);

    update_task::spawn(Msg::Navigate(Screen::Home)).unwrap();
    render_task::spawn().unwrap();

    loop {
      if let Err(_err) = render_task::spawn() {
        defmt::info!("error rendering in loop");
      }

      delay.delay_ms(30u16);
    }
  }

  #[task(priority = 3, local = [last_keys, delay2], shared = [keypad, state])]
  fn keypad_task(ctx: keypad_task::Context) {
    let keypad_task::LocalResources { last_keys, delay2 } = ctx.local;
    let keypad_task::SharedResources {
      mut keypad,
      mut state,
    } = ctx.shared;

    keypad.lock(|keypad| {
      if let Some(key) = keypad.read(delay2) {
        last_keys[0] = last_keys[1];
        last_keys[1] = last_keys[2];
        last_keys[2] = last_keys[3];
        last_keys[3] = last_keys[4];
        last_keys[4] = Some(key);
      } else {
        last_keys[0] = last_keys[1];
        last_keys[1] = last_keys[2];
        last_keys[2] = last_keys[3];
        last_keys[3] = last_keys[4];
        last_keys[4] = None;
      }
    });

    state.lock(|state| {
      match state.keypad_mode {
        KeypadMode::Number => {
          if let Some(button) = last_keys[4] {
            let key = match button {
              Button::Zero => Key::Zero,
              Button::One => Key::One,
              Button::Two => Key::Two,
              Button::Three => Key::Three,
              Button::Four => Key::Four,
              Button::Five => Key::Five,
              Button::Six => Key::Six,
              Button::Seven => Key::Seven,
              Button::Eight => Key::Eight,
              Button::Nine => Key::Nine,
              Button::Back => Key::Back,
              Button::Forward => Key::Forward,
            };
            update_task::spawn(Msg::Type(key)).unwrap();
          }
        }
        KeypadMode::Text => {
          let mut key = None;

          if last_keys[0].is_none()
            && last_keys[1].is_none()
            && last_keys[2].is_some()
            && last_keys[3].is_none()
            && last_keys[4].is_none()
          {
            // _ _ * _ _
            key = match last_keys[2].unwrap() {
              Button::Zero => Some(Key::Z),
              Button::One => None,
              Button::Two => Some(Key::A),
              Button::Three => Some(Key::D),
              Button::Four => Some(Key::G),
              Button::Five => Some(Key::J),
              Button::Six => Some(Key::M),
              Button::Seven => Some(Key::P),
              Button::Eight => Some(Key::T),
              Button::Nine => Some(Key::W),
              Button::Back => Some(Key::Back),
              Button::Forward => Some(Key::Forward),
            };
          } else if last_keys[0].is_none()
            && last_keys[1].is_some()
            && last_keys[2].is_some()
            && last_keys[3].is_none()
            && last_keys[4].is_none()
          {
            // _ * * _ _
            key = match last_keys[2].unwrap() {
              Button::Zero => None,
              Button::One => None,
              Button::Two => Some(Key::B),
              Button::Three => Some(Key::E),
              Button::Four => Some(Key::H),
              Button::Five => Some(Key::K),
              Button::Six => Some(Key::N),
              Button::Seven => Some(Key::R),
              Button::Eight => Some(Key::U),
              Button::Nine => Some(Key::X),
              Button::Back => Some(Key::Back),
              Button::Forward => Some(Key::Forward),
            };
          } else if last_keys[0].is_some()
            && last_keys[1].is_some()
            && last_keys[2].is_some()
            && last_keys[3].is_none()
            && last_keys[4].is_none()
          {
            // * * * _ _
            key = match last_keys[2].unwrap() {
              Button::Zero => None,
              Button::One => None,
              Button::Two => Some(Key::C),
              Button::Three => Some(Key::F),
              Button::Four => Some(Key::I),
              Button::Five => Some(Key::L),
              Button::Six => Some(Key::O),
              Button::Seven => Some(Key::S),
              Button::Eight => Some(Key::V),
              Button::Nine => Some(Key::Y),
              Button::Back => Some(Key::Back),
              Button::Forward => Some(Key::Forward),
            };
          }

          if let Some(key) = key {
            update_task::spawn(Msg::Type(key)).unwrap();
          }
        }
      }
    });

    keypad_task::spawn_after(50.milliseconds()).unwrap();
  }

  fn clear_screen(display: &mut Display, backlight: &mut BacklightLED) {
    backlight.set_low().unwrap();
    display.clear(Rgb565::BLACK).unwrap();
  }
  fn show_screen(backlight: &mut BacklightLED) {
    backlight.set_high().unwrap();
  }

  #[task(priority = 2, shared = [state, should_render] , local = [display, backlight])]
  fn render_task(ctx: render_task::Context) {
    let render_task::SharedResources {
      should_render,
      state,
    } = ctx.shared;
    let render_task::LocalResources { display, backlight } = ctx.local;

    (should_render, state).lock(|should_render, state| {
      if *should_render {
        clear_screen(display, backlight);

        view(display, &state);

        show_screen(backlight);

        *should_render = false;
      }
    });
  }

  #[task(priority = 1, shared = [state, should_render])]
  fn update_task(ctx: update_task::Context, msg: Msg) {
    defmt::info!("update!");
    let update_task::SharedResources {
      should_render,
      state,
    } = ctx.shared;

    (should_render, state).lock(|should_render, state| {
      update(state, msg);

      *should_render = true;
    });
  }
}
