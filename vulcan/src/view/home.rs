use super::util::{draw_button, ViewError, ViewResult, MARGIN_X, MARGIN_Y};
use crate::types::Model;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

pub fn home(
  display: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  state: &Model,
) -> ViewResult {
  display.clear(Rgb565::WHITE)?;

  let mut top_left = Point::new(MARGIN_X as i32, MARGIN_Y as i32);
  for (i, msg) in state.home_menu.iter().enumerate() {
    top_left = draw_button(display, &state, &top_left, i, msg)?;
  }

  Ok(())
}
