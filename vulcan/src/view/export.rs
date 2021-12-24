use super::util::{draw_button, ViewError, ViewResult};
use crate::types::{ExportScreen, Model};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use qr_code::QrCode;

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
    ExportScreen::SeedQR => {
      let seedqr = "136400980811079503490561095703230934105802751813017212440282184807481683015201310078178605500063";

      defmt::info!("{}", seedqr);

      let qr_code = QrCode::new(seedqr.as_bytes()).unwrap();
      let qr = qr_code.to_string(false, 3);
      let s = qr.as_str();
      defmt::info!("{}", s);

      // let raw_image_data = ImageRawLE::new(&qr_code.to_string(false, 3), 240);
      // Image::new(
      //   &raw_image_data,
      //   display.bounding_box().center() - Point::new(50, 50),
      // )
      // .draw(display)?;
    }
    _ => {}
  }

  Ok(())
}
