use super::util::{get_fonts, ViewError, ViewResult};
use crate::types::Model;
use embedded_graphics::{
  pixelcolor::Rgb565,
  prelude::*,
  text::{Alignment, Text},
};

pub fn create_wallet(
  display: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  state: &Model,
) -> ViewResult {
  display.clear(Rgb565::WHITE)?;

  let fonts = get_fonts();

  Text::with_alignment(
    "create wallet",
    display.bounding_box().center() + Point::new(0, 5),
    fonts.black,
    Alignment::Center,
  )
  .draw(display)?;

  Ok(())
}
