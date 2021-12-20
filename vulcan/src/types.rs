use crate::keypad::{Key, NavigationKey, NumberKey};
use display_interface_spi::SPIInterface;
use heapless::String;
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
pub enum KeypadMode {
  Number,
  Text,
  Navigation,
}

#[derive(Debug)]
pub enum KeyType {
  Number(NumberKey),
  Text(Key),
  Navigation(NavigationKey),
}

#[derive(Debug)]
pub struct Model {
  pub screen: Screen,
  pub msg: String<50usize>,
  pub keypad_mode: KeypadMode,
  pub selected_item: usize,
  pub home_menu: [&'static str; 4],
}
impl Default for Model {
  fn default() -> Self {
    return Self {
      screen: Screen::Splash,
      msg: String::from("home screen"),
      keypad_mode: KeypadMode::Navigation,
      selected_item: 0,
      home_menu: [
        "Create New Wallet",
        "Sign Transaction",
        "Verify Address",
        "Export Wallet",
      ],
    };
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportScreen {
  Menu,
  SeedQR,
  Specter,
  Sparrow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
  Splash,
  Home,
  Create,
  Sign,
  Verify,
  ExportWallet(ExportScreen),
}

#[derive(Debug)]
pub enum Msg {
  Navigate(Screen),
  KeyUp(KeyType),
}

pub enum Cmd {
  None,
  UpdateAfter(u32, Msg),
}
