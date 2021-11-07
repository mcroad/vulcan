use display_interface_spi::SPIInterface;
use embedded_graphics::draw_target::DrawTarget;
use heapless::String;
use keypad2::Keypad;
use rtic::time::Instant;
use st7789::ST7789;
use stm32h7xx_hal::{
  device::SPI1,
  gpio::{
    gpioa::{PA0, PA1, PA2, PA3},
    gpioe::{PE10, PE11, PE12, PE13, PE14, PE8, PE9},
    Input, OpenDrain, Output, PullUp, PushPull,
  },
  spi::{Enabled, Spi},
};
use systick_monotonic::Systick;

use crate::util::{Button, Key};

pub type Keys = Keypad<
  PE11<Input<PullUp>>,
  PE12<Input<PullUp>>,
  PE13<Input<PullUp>>,
  PE14<Input<PullUp>>,
  PE8<Output<OpenDrain>>,
  PE9<Output<OpenDrain>>,
  PE10<Output<OpenDrain>>,
>;

pub type Display = ST7789<
  SPIInterface<Spi<SPI1, Enabled, u8>, PA0<Output<PushPull>>, PA3<Output<PushPull>>>,
  PA2<Output<PushPull>>,
>;
pub type BacklightLED = PA1<Output<PushPull>>;

#[derive(Debug)]
pub enum Screen {
  Splash,
  Home,
}

#[derive(Debug)]
pub enum KeypadMode {
  Number,
  Text,
}

#[derive(Debug)]
pub struct State {
  pub screen: Screen,
  pub msg: String<50usize>,
  pub keypad_mode: KeypadMode,
}

#[derive(Debug)]
pub enum Msg {
  Navigate(Screen),
  Type(Key),
}

type PageViewError = <Display as DrawTarget>::Error;
pub type PageViewResult = Result<(), PageViewError>;

pub trait Page {
  fn update(_state: &mut State, _msg: Msg) {}
  fn view(display: &mut Display, state: &State) -> PageViewResult;
}

#[derive(Clone, Copy)]
pub struct ButtonEvent {
  pub button: Option<Button>,
  pub now: Instant<Systick<100>>,
}
impl ButtonEvent {
  pub fn is_some(&self) -> bool {
    self.button.is_some()
  }
  pub fn is_none(&self) -> bool {
    self.button.is_none()
  }
}
