#![no_main]
#![no_std]
#![deny(unused_imports)]
#![deny(unsafe_code)]
#![deny(warnings)]

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

mod home;
mod splash;
mod types;
mod update;
mod util;
mod view;

#[rtic::app(device = stm32h7xx_hal::stm32, peripherals = true, dispatchers = [USART1, USART2, EXTI0])]
mod app {
  use crate::types::{BacklightLED, ButtonEvent, Display, KeypadMode, Keys, Msg, Screen, State};
  use crate::update::update;
  use crate::util::{Button, Key, KeypadRead};
  use crate::view::view;
  use asm_delay::{bitrate, AsmDelay};
  use display_interface_spi::SPIInterface;
  use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
  use embedded_hal::spi::{Mode, Phase, Polarity};
  use embedded_hal::{digital::v2::OutputPin, prelude::*};
  use heapless::String;
  use keypad2::Keypad;
  use rtic::time::duration::*;
  use st7789::{Orientation, ST7789};
  use stm32h7xx_hal::prelude::*;
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
    last_buttons: [ButtonEvent; 20],
  }

  #[init]
  fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
    defmt::info!("INIT");
    let pwr = ctx.device.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Set up the system clock.
    let rcc = ctx.device.RCC.constrain();
    let ccdr = rcc
      .sys_ck(96.mhz())
      .pll1_q_ck(48.mhz())
      .freeze(pwrcfg, &ctx.device.SYSCFG);

    let mono = Systick::<100>::new(ctx.core.SYST, 100_000_000);

    let gpioa = ctx.device.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpioe = ctx.device.GPIOE.split(ccdr.peripheral.GPIOE);

    let mut delay = AsmDelay::new(bitrate::MegaHertz(100));

    let mut display = {
      let sck1 = gpioa.pa5.into_alternate_af5();
      let miso1 = gpioa.pa6.into_alternate_af5();
      let mosi1 = gpioa.pa7.into_alternate_af5();
      let spi1 = ctx.device.SPI1.spi(
        (sck1, miso1, mosi1),
        Mode {
          polarity: Polarity::IdleLow,
          phase: Phase::CaptureOnFirstTransition,
        },
        8.mhz(),
        ccdr.peripheral.SPI1,
        &ccdr.clocks,
      );

      let chip_select = gpioa.pa3.into_push_pull_output(); // chip select
      let data_control = gpioa.pa0.into_push_pull_output(); // data control
      let display_interface = SPIInterface::new(spi1, data_control, chip_select);

      let lcd_reset = gpioa.pa2.into_push_pull_output();

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
    let mut backlight = gpioa.pa1.into_push_pull_output();
    backlight.set_high().unwrap();

    let keypad = {
      let cols = (
        gpioe.pe8.into_open_drain_output(),
        gpioe.pe9.into_open_drain_output(),
        gpioe.pe10.into_open_drain_output(),
      );
      let rows = (
        gpioe.pe11.into_pull_up_input(),
        gpioe.pe12.into_pull_up_input(),
        gpioe.pe13.into_pull_up_input(),
        gpioe.pe14.into_pull_up_input(),
      );

      Keypad::new(rows, cols)
    };

    let now = monotonics::now();
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
        last_buttons: [ButtonEvent { button: None, now }; 20],
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

    keypad_task::spawn().unwrap();

    loop {
      if let Err(_err) = render_task::spawn() {
        defmt::info!("error rendering in loop");
      }

      delay.delay_ms(30u16);
    }
  }

  struct ButtonKeyMap {
    two: [Key; 3],
    three: [Key; 3],
    four: [Key; 3],
    five: [Key; 3],
    six: [Key; 3],
    seven: [Key; 4],
    eight: [Key; 3],
    nine: [Key; 4],
  }

  static BUTTON_KEY_MAP: ButtonKeyMap = ButtonKeyMap {
    two: [Key::A, Key::B, Key::C],
    three: [Key::D, Key::E, Key::F],
    four: [Key::G, Key::H, Key::I],
    five: [Key::J, Key::K, Key::L],
    six: [Key::M, Key::N, Key::O],
    seven: [Key::P, Key::Q, Key::R, Key::S],
    eight: [Key::T, Key::U, Key::V],
    nine: [Key::W, Key::X, Key::Y, Key::Z],
  };

  fn button_to_key(button: Button, times_pressed: usize) -> Option<Key> {
    if times_pressed < 1 {
      panic!("button_to_key: times_pressed out of range");
    }
    if button == Button::Seven || button == Button::Nine {
      if times_pressed > 4 {
        panic!("button_to_key: times_pressed out of range");
      }
    } else if times_pressed > 3 {
      panic!("button_to_key: times_pressed out of range");
    }

    match button {
      Button::Zero => None,
      Button::One => None,
      Button::Two => Some(BUTTON_KEY_MAP.two[times_pressed]),
      Button::Three => Some(BUTTON_KEY_MAP.three[times_pressed]),
      Button::Four => Some(BUTTON_KEY_MAP.four[times_pressed]),
      Button::Five => Some(BUTTON_KEY_MAP.five[times_pressed]),
      Button::Six => Some(BUTTON_KEY_MAP.six[times_pressed]),
      Button::Seven => Some(BUTTON_KEY_MAP.seven[times_pressed]),
      Button::Eight => Some(BUTTON_KEY_MAP.eight[times_pressed]),
      Button::Nine => Some(BUTTON_KEY_MAP.nine[times_pressed]),
      Button::Back => Some(Key::Back),
      Button::Forward => Some(Key::Forward),
    }
  }

  fn push_button_press(keys: &mut [ButtonEvent], value: ButtonEvent) {
    let len = keys.len();
    for i in 1..len {
      keys[i] = keys[i - 1];
    }
    keys[0] = value;
  }

  fn check_for_triple(keys: &[ButtonEvent]) -> bool {
    // let some_buttons = keys.iter().filter(|ev| ev.is_some());

    if keys[1].is_some()
      && keys[2].is_none()
      && keys[3].is_some()
      && keys[4].is_none()
      && keys[5].is_some()
      && keys[6].is_none()
    {
      if keys[1].button == keys[3].button && keys[3].button == keys[5].button {
        return true;
      }
    }

    return false;
  }
  // fn check_last_2(keys: &[ButtonEvent]) -> bool {
  //   if keys[1].is_some() && keys[2].is_none() && keys[3].is_some() && keys[4].is_none() {
  //     if keys[1].button == keys[3].button {
  //       return true;
  //     }
  //   }

  //   return false;
  // }

  #[task(priority = 3, local = [last_buttons, delay2], shared = [keypad, state])]
  fn keypad_task(ctx: keypad_task::Context) -> () {
    let keypad_task::LocalResources {
      last_buttons,
      delay2,
    } = ctx.local;
    let keypad_task::SharedResources {
      mut keypad,
      mut state,
    } = ctx.shared;

    let now = monotonics::now();

    keypad.lock(|keypad| {
      let read_button = keypad.read(delay2);

      push_button_press(
        last_buttons,
        ButtonEvent {
          button: read_button,
          now,
        },
      );

      // if let Some(button) = read_button {
      //   if last_buttons[0].is_none() {
      //     push_button_press(
      //       last_buttons,
      //       ButtonEvent {
      //         button: Some(button),
      //         now,
      //       },
      //     );
      //   }
      // } else {
      //   if last_buttons[0].is_some() {
      //     push_button_press(last_buttons, ButtonEvent { button: None, now });
      //   }
      // }
    });

    state.lock(|state| {
      match state.keypad_mode {
        KeypadMode::Number => {
          if let Some(button) = last_buttons[4].button {
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
          if last_buttons[0].is_some() {
            // wait until next cycle
            return;
          }

          let mut key: Option<Key> = None;

          if check_for_triple(last_buttons) {
            // last 3 are the same
            key = button_to_key(last_buttons[1].button.unwrap(), 3);
          }

          if last_buttons[1].is_some()
            && last_buttons[2].is_none()
            && last_buttons[3].is_none()
            && last_buttons[4].is_none()
          {
            // _ * _ _ _ _
            key = button_to_key(last_buttons[1].button.unwrap(), 1);
          } else if last_buttons[1].is_some()
            && last_buttons[2].is_none()
            && last_buttons[3].is_some()
            && last_buttons[4].is_none()
            && last_buttons[5].is_none()
            && last_buttons[6].is_none()
          {
            // _ * _ * _ _ _
            key = button_to_key(last_buttons[1].button.unwrap(), 2);
          } else if last_buttons[1].is_some()
            && last_buttons[2].is_none()
            && last_buttons[3].is_some()
            && last_buttons[4].is_none()
            && last_buttons[5].is_some()
            && last_buttons[6].is_none()
            && last_buttons[7].is_none()
          {
            // _ * _ * _ * _ _ _
            key = button_to_key(last_buttons[1].button.unwrap(), 3);
          } else if last_buttons[1].is_some()
            && last_buttons[2].is_some()
            && last_buttons[3].is_none()
            && last_buttons[4].is_none()
          {
            // _ * _ * _ * _ * _ _ _
            key = button_to_key(last_buttons[1].button.unwrap(), 4);
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
