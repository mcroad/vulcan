use crate::types::{Display, Page, PageViewResult, State};
use embedded_graphics::{
  mono_font::{ascii::FONT_10X20, MonoTextStyle},
  pixelcolor::Rgb565,
  prelude::*,
  text::{Alignment, Text},
};

pub struct Home {}
impl Page for Home {
  fn view(display: &mut Display, state: &State) -> PageViewResult {
    display.clear(Rgb565::WHITE)?;

    let character_style = MonoTextStyle::new(&FONT_10X20, Rgb565::BLACK);

    // Draw centered text.
    Text::with_alignment(
      &state.msg,
      display.bounding_box().center() + Point::new(0, 15),
      character_style,
      Alignment::Center,
    )
    .draw(display)?;

    return Ok(());
  }
}
