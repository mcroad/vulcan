use embedded_graphics::{
  mono_font::{ascii::FONT_10X20, MonoTextStyle},
  pixelcolor::Rgb565,
  prelude::*,
  primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
  text::{Alignment, Text},
};
use stm32h7xx_hal::Never;
use crate::types::{Display, Model};

pub type ViewColor = <Display as DrawTarget>::Color;
pub type ViewError = st7789::Error<Never>;
pub type ViewResult = Result<(), ViewError>;

pub struct Fonts<'a> {
  pub black: MonoTextStyle<'a, Rgb565>,
  pub white: MonoTextStyle<'a, Rgb565>,
}

pub fn get_fonts() -> Fonts<'static> {
  return Fonts {
    black: MonoTextStyle::new(&FONT_10X20, Rgb565::BLACK),
    white: MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE),
  };
}

fn rectangle_style(item_n: usize, cur_item: usize) -> PrimitiveStyle<Rgb565> {
  let mut style = PrimitiveStyleBuilder::new()
    .stroke_width(1)
    .stroke_color(Rgb565::BLACK);

  if item_n == cur_item {
    style = style.fill_color(Rgb565::new(255, 165, 0));
  }

  style.build()
}

pub fn draw_button(
  display: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  state: &Model,
  top_left: &Point,
  width: u32,
  button_n: usize,
  text: &str,
) -> Result<Point, ViewError> {
  let fonts = get_fonts();
  let corner_radius = Size::new(10, 10);
  let rectangle_size = Size::new(width, 30);

  let rec =
    RoundedRectangle::with_equal_corners(Rectangle::new(*top_left, rectangle_size), corner_radius)
      .into_styled(rectangle_style(button_n, state.selected_item));
  rec.draw(display)?;

  // Draw centered text.
  Text::with_alignment(
    text,
    rec.bounding_box().center() + Point::new(0, 5),
    if button_n == state.selected_item {
      fonts.white
    } else {
      fonts.black
    },
    Alignment::Center,
  )
  .draw(display)?;

  let margin = 10;
  let space = 10;

  return Ok(Point::new(
    margin,
    top_left.y + rectangle_size.height as i32 + space,
  ));
}
