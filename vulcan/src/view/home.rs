use super::util::{draw_button, ViewError, ViewResult};
use crate::types::Model;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

pub fn home(
  display: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  state: &Model,
) -> ViewResult {
  display.clear(Rgb565::WHITE)?;

  let margin = 10;
  let space = 10;

  let width: u32 = display.bounding_box().bottom_right().unwrap().x as u32 - (margin as u32) * 2;

  let mut top_left = Point::new(margin, space);
  for (i, msg) in state.home_menu.iter().enumerate() {
    top_left = draw_button(display, &state, &top_left, width, i, msg)?;
  }

  Ok(())
}
