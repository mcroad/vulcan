use core::convert::TryInto;
use embedded_hal::blocking::delay::DelayMs;
use keypad2::Keypad;
use rtic_monotonic::{Instant, Milliseconds};
use stm32h7xx_hal::gpio::{
  gpiob::PB1,
  gpioc::PC5,
  gpioe::{PE11, PE13, PE15, PE7, PE9},
  Input, OpenDrain, Output, PullUp,
};
use systick_monotonic::Systick;

pub type Keys = Keypad<
  PE9<Input<PullUp>>,
  PE11<Input<PullUp>>,
  PE13<Input<PullUp>>,
  PE15<Input<PullUp>>,
  PC5<Output<OpenDrain>>,
  PB1<Output<OpenDrain>>,
  PE7<Output<OpenDrain>>,
>;
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
  Up,
  Down,
  Left,
  Right,
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
      Key::Up => "^",
      Key::Down => "\\/",
      Key::Left => "<",
      Key::Right => ">",
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
      Key::Up => defmt::write!(f, "^"),
      Key::Down => defmt::write!(f, "\\/"),
      Key::Left => defmt::write!(f, "<"),
      Key::Right => defmt::write!(f, ">"),
    }
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

  pub fn to_key(&self, times_pressed: usize) -> Option<Key> {
    if times_pressed < 1 {
      panic!("button_to_key: times_pressed out of range");
    }
    if *self == Button::Seven || *self == Button::Nine {
      if times_pressed > 4 {
        panic!("button_to_key: times_pressed out of range");
      }
    } else if times_pressed > 3 {
      panic!("button_to_key: times_pressed out of range");
    }

    let i = times_pressed - 1;

    match *self {
      Button::Zero => Some(Key::Zero),
      Button::One => None,
      Button::Two => Some(BUTTON_KEY_MAP.two[i]),
      Button::Three => Some(BUTTON_KEY_MAP.three[i]),
      Button::Four => Some(BUTTON_KEY_MAP.four[i]),
      Button::Five => Some(BUTTON_KEY_MAP.five[i]),
      Button::Six => Some(BUTTON_KEY_MAP.six[i]),
      Button::Seven => Some(BUTTON_KEY_MAP.seven[i]),
      Button::Eight => Some(BUTTON_KEY_MAP.eight[i]),
      Button::Nine => Some(BUTTON_KEY_MAP.nine[i]),
      Button::Back => Some(Key::Back),
      Button::Forward => Some(Key::Forward),
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

#[derive(Clone, Copy)]
pub struct ButtonEvent {
  pub button: Option<Button>,
  pub now: Instant<Systick<480>>,
}
impl ButtonEvent {
  pub fn is_some(&self) -> bool {
    self.button.is_some()
  }
  pub fn is_none(&self) -> bool {
    self.button.is_none()
  }
}
impl defmt::Format for ButtonEvent {
  fn format(&self, f: defmt::Formatter) {
    defmt::write!(f, "ButtonEvent<{}>", self.button);
  }
}

pub type EventBuffer = [ButtonEvent; 8];
pub trait EventBufferUtil {
  fn unshift(&mut self, event: ButtonEvent) -> ();
  fn check_for_quad(&self) -> bool;
  fn check_for_triple(&self) -> bool;
  fn check_for_double(&self) -> bool;
  fn check_for_single(&self) -> bool;
}
impl EventBufferUtil for EventBuffer {
  /// adds events to the front of the que
  fn unshift(&mut self, event: ButtonEvent) {
    let len = self.len();
    // move every event back 1
    for i in (1..len).rev() {
      self[i] = self[i - 1];
    }
    self[0] = event;
  }

  fn check_for_quad(&self) -> bool {
    if self[1].is_some()
      && self[2].is_none()
      && self[3].is_some()
      && self[4].is_none()
      && self[5].is_some()
      && self[6].is_none()
      && self[7].is_some()
    {
      if self[1].button == self[3].button
        && self[3].button == self[5].button
        && self[5].button == self[7].button
      {
        let button = self[1].button.unwrap();
        if button == Button::Seven || button == Button::Nine {
          return true;
        }
      }
    }

    return false;
  }

  fn check_for_triple(&self) -> bool {
    if self[1].is_some()
      && self[2].is_none()
      && self[3].is_some()
      && self[4].is_none()
      && self[5].is_some()
      && self[6].is_none()
    {
      if self[1].button == self[3].button && self[3].button == self[5].button {
        return true;
      }
    }

    return false;
  }

  fn check_for_double(&self) -> bool {
    if self[1].is_some() && self[2].is_none() && self[3].is_some() && self[4].is_none() {
      if self[1].button == self[3].button {
        return true;
      }
    }

    return false;
  }

  fn check_for_single(&self) -> bool {
    if self[0].is_none() && self[1].is_some() && self[2].is_none() {
      return true;
    }

    return false;
  }
}

/// checks that enough time has passed between 2 instants
pub fn check_timespan_ms(
  first: &Instant<Systick<480>>,
  second: &Instant<Systick<480>>,
  timespan: u32,
) -> bool {
  let generic_duration = second.checked_duration_since(&first).unwrap();

  let millis: Milliseconds<u32> = generic_duration.try_into().unwrap();
  if millis > Milliseconds(timespan) {
    return true;
  }

  return false;
}
