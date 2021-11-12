use crate::types::Model;
use crate::view::{ViewError, ViewResult};
use embedded_graphics::primitives::{
  PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle,
};
use embedded_graphics::{
  mono_font::{ascii::FONT_10X20, MonoTextStyle},
  pixelcolor::Rgb565,
  prelude::*,
  text::{Alignment, Text},
};

fn rectangle_style(item_n: usize, cur_item: usize) -> PrimitiveStyle<Rgb565> {
  let mut style = PrimitiveStyleBuilder::new()
    .stroke_width(1)
    .stroke_color(Rgb565::BLACK);

  if item_n == cur_item {
    style = style.fill_color(Rgb565::new(255, 165, 0));
  }

  style.build()
}

pub fn home(
  display: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  state: &Model,
) -> ViewResult {
  display.clear(Rgb565::WHITE)?;

  let msgs = ["item 1", "item 2", "item 3", "item 4", "item 5"];

  let black = MonoTextStyle::new(&FONT_10X20, Rgb565::BLACK);
  let white = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);

  let margin = 10;
  let space = 10;

  let corner_radius = Size::new(10, 10);
  let width: u32 = display.bounding_box().bottom_right().unwrap().x as u32 - (margin as u32) * 2;
  let rectangle_size = Size::new(width, 30);

  let mut top_left = Point::new(margin, space);
  for i in 0..5 {
    let rec =
      RoundedRectangle::with_equal_corners(Rectangle::new(top_left, rectangle_size), corner_radius)
        .into_styled(rectangle_style(i, state.selected_item));
    rec.draw(display)?;

    // Draw centered text.
    Text::with_alignment(
      msgs[i],
      rec.bounding_box().center() + Point::new(0, 5),
      if i == state.selected_item {
        white
      } else {
        black
      },
      Alignment::Center,
    )
    .draw(display)?;

    top_left = Point::new(margin, top_left.y + rectangle_size.height as i32 + space);
  }

  Ok(())
}
