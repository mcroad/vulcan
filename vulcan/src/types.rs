use crate::keypad::Key;
use display_interface_spi::SPIInterface;
use heapless::String;
use rtic_monotonic::Milliseconds;
use st7789::ST7789;
use stm32h7xx_hal::{
  device::SPI1,
  gpio::{
    gpioa::{PA0, PA1, PA2, PA3},
    Output, PushPull,
  },
  spi::{Enabled, Spi},
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
  Navigation,
}

#[derive(Debug)]
pub struct State {
  pub screen: Screen,
  pub msg: String<50usize>,
  pub keypad_mode: KeypadMode,
  pub selected_item: usize,
}

#[derive(Debug)]
pub enum Msg {
  Navigate(Screen),
  KeyUp(Key),
}

pub enum Cmd {
  Noop,
  UpdateAfter(Milliseconds, Msg),
}
