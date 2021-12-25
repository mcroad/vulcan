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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ExportScreen {
  Menu,
  SeedQR,
  Specter,
  Sparrow,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SignScreen {
  Menu,
  FromQR,
  FromFile,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Screen {
  Splash,
  Home,
  Create,
  Sign(SignScreen),
  Verify,
  ExportWallet(ExportScreen),
}

#[derive(Debug, PartialEq)]
pub enum Msg {
  Navigate(Screen),
  KeyUp(KeyType),
}

impl defmt::Format for Msg {
  fn format(&self, f: defmt::Formatter) {
    match self {
      Msg::Navigate(screen) => defmt::write!(f, "Msg::Navigate({})", defmt::Debug2Format(&screen)),
      Msg::KeyUp(key) => defmt::write!(f, "Msg::KeyUp({})", defmt::Debug2Format(&key)),
    }
  }
}

#[derive(PartialEq)]
pub enum Cmd {
  None,
  UpdateAfter(u64, Msg),
  InitSD,
}

impl defmt::Format for Cmd {
  fn format(&self, f: defmt::Formatter) {
    match self {
      Cmd::None => defmt::write!(f, "Cmd::None"),
      Cmd::InitSD => defmt::write!(f, "Cmd::InitSD"),
      Cmd::UpdateAfter(time, msg) => defmt::write!(f, "Cmd::UpdateAfter({}, {})", time, msg),
    }
  }
}
