use crate::{
  types::Model,
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
  _state: &Model,
) -> ViewResult {
  // volcano icon adapted from https://www.flaticon.com/premium-icon/volcano_2076995
  let raw_image_data = ImageRawLE::new(include_bytes!("../../assets/volcano.raw"), 100);
  let volcano = Image::new(
    &raw_image_data,
    display.bounding_box().center() - Point::new(50, 50),
  );

  let white_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
  Text::with_alignment(
    "Vulcan",
    display.bounding_box().center() - Point::new(0, 90),
    white_style,
    Alignment::Center,
  )
  .draw(display)?;

  volcano.draw(display)?;

  Text::with_alignment(
    env!("GIT_HASH"),
    display.bounding_box().center() + Point::new(0, 90),
    white_style,
    Alignment::Center,
  )
  .draw(display)?;

  return Ok(());
}
