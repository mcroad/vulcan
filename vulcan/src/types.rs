use crate::keypad::Key;
use display_interface_spi::SPIInterface;
use embedded_graphics::draw_target::DrawTarget;
use heapless::String;
use st7789::ST7789;
use stm32h7xx_hal::{
  device::SPI1,
  gpio::{
    gpioa::{PA0, PA1, PA2, PA3},
    Output, PushPull,
  },
  spi::{Enabled, Spi},
  Never,
};

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
  KeyUp(Key),
}

pub type ViewColor = <Display as DrawTarget>::Color;
pub type ViewError = st7789::Error<Never>;
pub type PageViewResult = Result<(), ViewError>;

pub trait Page {
  fn update(_state: &mut State, _msg: Msg) {}
  fn view(
    display: &mut impl DrawTarget<Color = ViewColor, Error = ViewError>,
    state: &State,
  ) -> PageViewResult;
}
