use crate::types::Keys;
use embedded_hal::blocking::delay::DelayMs;
pub use stm32f4xx_hal::delay::Delay;

#[derive(Debug, Clone, Copy)]
pub enum Key {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  Zero,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Back,
  Forward,
}

impl Key {
  pub fn to_string(&self) -> &str {
    match *self {
      Key::A => "A",
      Key::B => "B",
      Key::C => "C",
      Key::D => "D",
      Key::E => "E",
      Key::F => "F",
      Key::G => "G",
      Key::H => "H",
      Key::I => "I",
      Key::J => "J",
      Key::K => "K",
      Key::L => "L",
      Key::M => "M",
      Key::N => "N",
      Key::O => "O",
      Key::P => "P",
      Key::Q => "Q",
      Key::R => "R",
      Key::S => "S",
      Key::T => "T",
      Key::U => "U",
      Key::V => "V",
      Key::W => "W",
      Key::X => "X",
      Key::Y => "Y",
      Key::Z => "Z",
      Key::Zero => "0",
      Key::One => "1",
      Key::Two => "2",
      Key::Three => "3",
      Key::Four => "4",
      Key::Five => "5",
      Key::Six => "6",
      Key::Seven => "7",
      Key::Eight => "8",
      Key::Nine => "9",
      Key::Back => "*",
      Key::Forward => "#",
    }
  }
}
impl defmt::Format for Key {
  fn format(&self, f: defmt::Formatter) {
    match *self {
      Key::A => defmt::write!(f, "A"),
      Key::B => defmt::write!(f, "B"),
      Key::C => defmt::write!(f, "C"),
      Key::D => defmt::write!(f, "D"),
      Key::E => defmt::write!(f, "E"),
      Key::F => defmt::write!(f, "F"),
      Key::G => defmt::write!(f, "G"),
      Key::H => defmt::write!(f, "H"),
      Key::I => defmt::write!(f, "I"),
      Key::J => defmt::write!(f, "J"),
      Key::K => defmt::write!(f, "K"),
      Key::L => defmt::write!(f, "L"),
      Key::M => defmt::write!(f, "M"),
      Key::N => defmt::write!(f, "N"),
      Key::O => defmt::write!(f, "O"),
      Key::P => defmt::write!(f, "P"),
      Key::Q => defmt::write!(f, "Q"),
      Key::R => defmt::write!(f, "R"),
      Key::S => defmt::write!(f, "S"),
      Key::T => defmt::write!(f, "T"),
      Key::U => defmt::write!(f, "U"),
      Key::V => defmt::write!(f, "V"),
      Key::W => defmt::write!(f, "W"),
      Key::X => defmt::write!(f, "X"),
      Key::Y => defmt::write!(f, "Y"),
      Key::Z => defmt::write!(f, "Z"),
      Key::Zero => defmt::write!(f, "0"),
      Key::One => defmt::write!(f, "1"),
      Key::Two => defmt::write!(f, "2"),
      Key::Three => defmt::write!(f, "3"),
      Key::Four => defmt::write!(f, "4"),
      Key::Five => defmt::write!(f, "5"),
      Key::Six => defmt::write!(f, "6"),
      Key::Seven => defmt::write!(f, "7"),
      Key::Eight => defmt::write!(f, "8"),
      Key::Nine => defmt::write!(f, "9"),
      Key::Back => defmt::write!(f, "*"),
      Key::Forward => defmt::write!(f, "#"),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Button {
  Zero,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Back,
  Forward,
}
impl Button {
  pub fn to_string(&self) -> &str {
    match *self {
      Button::Zero => "0",
      Button::One => "1",
      Button::Two => "2",
      Button::Three => "3",
      Button::Four => "4",
      Button::Five => "5",
      Button::Six => "6",
      Button::Seven => "7",
      Button::Eight => "8",
      Button::Nine => "9",
      Button::Back => "*",
      Button::Forward => "#",
    }
  }
}
impl defmt::Format for Button {
  fn format(&self, f: defmt::Formatter) {
    match *self {
      Button::Zero => defmt::write!(f, "0"),
      Button::One => defmt::write!(f, "1"),
      Button::Two => defmt::write!(f, "2"),
      Button::Three => defmt::write!(f, "3"),
      Button::Four => defmt::write!(f, "4"),
      Button::Five => defmt::write!(f, "5"),
      Button::Six => defmt::write!(f, "6"),
      Button::Seven => defmt::write!(f, "7"),
      Button::Eight => defmt::write!(f, "8"),
      Button::Nine => defmt::write!(f, "9"),
      Button::Back => defmt::write!(f, "*"),
      Button::Forward => defmt::write!(f, "#"),
    }
  }
}

pub trait KeypadRead {
  fn read(&mut self, delay: &mut dyn DelayMs<u16>) -> Option<Button>;
}
impl KeypadRead for Keys {
  fn read(&mut self, delay: &mut dyn DelayMs<u16>) -> Option<Button> {
    let key = self.read_char(delay);
    match key {
      '0' => Some(Button::Zero),
      '1' => Some(Button::One),
      '2' => Some(Button::Two),
      '3' => Some(Button::Three),
      '4' => Some(Button::Four),
      '5' => Some(Button::Five),
      '6' => Some(Button::Six),
      '7' => Some(Button::Seven),
      '8' => Some(Button::Eight),
      '9' => Some(Button::Nine),
      '*' => Some(Button::Back),
      '#' => Some(Button::Forward),
      _ => None,
    }
  }
}
