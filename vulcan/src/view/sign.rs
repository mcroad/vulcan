use super::util::{draw_button, draw_nav, ViewError, ViewResult, MARGIN_X, MARGIN_Y};
use crate::types::{Model, SignScreen};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

pub fn sign_transaction(
  display: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  state: &Model,
  screen: &SignScreen,
) -> ViewResult {
  display.clear(Rgb565::WHITE)?;

  match screen {
    SignScreen::Menu => {
      let mut top_left =
        draw_nav(display, "Sign Transaction")? + Point::new(MARGIN_X as i32, MARGIN_Y as i32);

      top_left = draw_button(display, &state, &top_left, 0, "Scan QR Code")?;
      draw_button(display, &state, &top_left, 1, "Select PSBT File")?;
    }
    SignScreen::FromFile => {
      let mut top_left =
        draw_nav(display, "Select PSBT File")? + Point::new(MARGIN_X as i32, MARGIN_Y as i32);

      let width: u32 = display.bounding_box().bottom_right().unwrap().x as u32 - MARGIN_X * 2;
      // top_left = draw_button(display, &state, &top_left, width, 0, "Scan QR Code")?;
      // top_left = draw_button(display, &state, &top_left, width, 1, "Select PSBT File")?;
    }
    _ => {}
  }

  Ok(())
}
