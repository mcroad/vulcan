use crate::{
  types::State,
  view::{ViewColor, ViewError, ViewResult},
};
use embedded_graphics::{
  image::{Image, ImageRawLE},
  mono_font::{ascii::FONT_10X20, MonoTextStyle},
  pixelcolor::Rgb565,
  prelude::*,
  text::{Alignment, Text},
};

pub fn splash(
  display: &mut impl DrawTarget<Color = ViewColor, Error = ViewError>,
  _state: &State,
) -> ViewResult {
  let white_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);

  Text::with_alignment(
    "Vulcan",
    display.bounding_box().center() - Point::new(0, 70),
    white_style,
    Alignment::Center,
  )
  .draw(display)?;

  let raw_image_data = ImageRawLE::new(include_bytes!("../../assets/ferris.raw"), 86);
  let volcano = Image::new(
    &raw_image_data,
    display.bounding_box().center() - Point::new(43, 40),
  );
  volcano.draw(display)?;

  Text::with_alignment(
    env!("GIT_HASH"),
    display.bounding_box().center() + Point::new(0, 60),
    white_style,
    Alignment::Center,
  )
  .draw(display)?;

  return Ok(());
}
