use display_interface_spi::SPIInterface;
use embedded_graphics::draw_target::DrawTarget;
use heapless::String;
use keypad2::Keypad;
use st7789::ST7789;
use stm32f4xx_hal::{
  gpio::{
    gpioa::{PA1, PA10, PA11, PA12, PA15, PA2, PA3, PA4, PA8, PA9},
    gpiob::{PB10, PB14, PB15, PB3},
    Alternate, Input, OpenDrain, Output, PullUp, PushPull, AF5,
  },
  spi::Spi,
  stm32::SPI2,
};

use crate::util::{Key};

pub type Keys = Keypad<
  PA11<Input<PullUp>>,
  PA12<Input<PullUp>>,
  PA15<Input<PullUp>>,
  PB3<Input<PullUp>>,
  PA8<Output<OpenDrain>>,
  PA9<Output<OpenDrain>>,
  PA10<Output<OpenDrain>>,
>;

pub type Display = ST7789<
  SPIInterface<
    Spi<
      SPI2,
      (
        PB10<Alternate<AF5>>,
        PB14<Alternate<AF5>>,
        PB15<Alternate<AF5>>,
      ),
    >,
    PA1<Output<PushPull>>,
    PA4<Output<PushPull>>,
  >,
  PA3<Output<PushPull>>,
>;
pub type BacklightLED = PA2<Output<PushPull>>;

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
