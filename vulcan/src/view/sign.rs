use super::util::get_fonts;
use crate::types::Model;
use crate::view::{ViewError, ViewResult};
use embedded_graphics::{
  pixelcolor::Rgb565,
  prelude::*,
  text::{Alignment, Text},
};

pub fn sign_transaction(
  display: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  state: &Model,
) -> ViewResult {
  display.clear(Rgb565::WHITE)?;

  let fonts = get_fonts();

  Text::with_alignment(
    "sign transaction",
    display.bounding_box().center() + Point::new(0, 5),
    fonts.black,
    Alignment::Center,
  )
  .draw(display)?;

  Ok(())
}
