use crate::types::State;
use crate::view::{ViewError, ViewResult};
use embedded_graphics::{
  mono_font::{ascii::FONT_10X20, MonoTextStyle},
  pixelcolor::Rgb565,
  prelude::*,
  text::{Alignment, Text},
};

pub fn home(
  display: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  state: &State,
) -> ViewResult {
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
