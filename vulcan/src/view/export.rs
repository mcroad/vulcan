use super::util::{draw_button, draw_nav, ViewError, ViewResult, MARGIN_X, MARGIN_Y};
use crate::{
  types::{ExportScreen, Model},
  view::util::ViewColor,
};
use alloc::vec;
use core::mem::size_of;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_qr::{QrCode, QrCodeEcc, QrDrawable, Version};

fn draw_qr(
  target: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  qr: &QrCode,
) -> ViewResult {
  const HEIGHT: usize = 240;
  if HEIGHT >= 2usize.pow((size_of::<usize>() * 4) as u32) {
    // error
    return Ok(());
  }

  let margin_size = 1;

  let s = qr.size();

  let data_length = s as usize;

  let data_length_with_margin = data_length + 2 * margin_size;

  let point_size = HEIGHT / data_length_with_margin;

  if point_size == 0 {
    // error
    return Ok(());
  }

  let margin = (HEIGHT - (point_size * data_length)) / 2;

  let mut buff = [false; HEIGHT * HEIGHT];
  for i in 0..s {
    for j in 0..s {
      if qr.get_module(i, j) {
        let x = i as usize * point_size + margin;
        let y = j as usize * point_size + margin;

        for j in y..(y + point_size) {
          let offset = j * HEIGHT;
          for i in x..(x + point_size) {
            buff[offset + i] = true;
          }
        }
      }
    }
  }

  let center = target.bounding_box().center();

  let pixels = buff.iter().enumerate().map(|(i, is_black)| {
    let i: i32 = i as i32;
    let y = i / 240;
    let x = i - (y * 240);
    let color = if *is_black {
      ViewColor::BLACK
    } else {
      ViewColor::WHITE
    };
    let x = x + (center.x / 4);
    Pixel(Point::new(x, y), color)
  });

  target.draw_iter(pixels)
}

pub fn export_wallet(
  display: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  state: &Model,
  screen: &ExportScreen,
) -> ViewResult {
  display.clear(Rgb565::WHITE)?;

  match screen {
    ExportScreen::Menu => {
      let mut top_left =
        draw_nav(display, "Export Wallet")? + Point::new(MARGIN_X as i32, MARGIN_Y as i32);

      top_left = draw_button(display, &state, &top_left, 0, "SeedQR")?;
      top_left = draw_button(display, &state, &top_left, 1, "Specter")?;
      draw_button(display, &state, &top_left, 2, "Sparrow")?;
    }
    ExportScreen::SeedQR => {
      let seedqr = "136400980811079503490561095703230934105802751813017212440282184807481683015201310078178605500063";

      defmt::info!("{}", seedqr);

      let version = Version::new(3);
      let mut outbuffer = vec![0u8; version.buffer_len()];
      let mut tempbuffer = vec![0u8; version.buffer_len()];
      let qr: QrCode = QrCode::encode_text(
        seedqr,
        &mut tempbuffer,
        &mut outbuffer,
        QrCodeEcc::Low,
        version,
        version,
        None,
        true,
      )
      .unwrap();
      // Note: qr has a reference to outbuffer, so outbuffer needs to outlive qr
      // Optional, because tempbuffer is only needed during encode_text()
      core::mem::drop(tempbuffer);

      let mut buff = [false; 240 * 240];
      let mut drawable: QrDrawable<'_, '_, Rgb565> = QrDrawable::new(&qr, &mut buff);
      drawable.prepare(240).unwrap();
      drawable.draw(display)?;

      core::mem::drop(outbuffer);
    }
    _ => {}
  }

  Ok(())
}
