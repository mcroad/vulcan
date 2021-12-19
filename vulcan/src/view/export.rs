use super::util::{draw_button, ViewError, ViewResult};
use crate::types::{ExportScreen, Model};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

pub fn export_wallet(
  display: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  state: &Model,
  export_screen: ExportScreen,
) -> ViewResult {
  display.clear(Rgb565::WHITE)?;

  match export_screen {
    ExportScreen::Menu => {
      let margin = 10;
      let space = 10;

      let width: u32 =
        display.bounding_box().bottom_right().unwrap().x as u32 - (margin as u32) * 2;

      let mut top_left = Point::new(margin, space);

      top_left = draw_button(display, &state, &top_left, width, 0, "SeedQR")?;
      top_left = draw_button(display, &state, &top_left, width, 1, "Specter")?;
      draw_button(display, &state, &top_left, width, 2, "Sparrow")?;
    }
    _ => {}
  }

  Ok(())
}
