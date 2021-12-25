use crate::types::{Display, Model};
use embedded_graphics::{
  mono_font::{ascii::FONT_10X20, MonoTextStyle},
  pixelcolor::Rgb565,
  prelude::*,
  primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
  text::{Alignment, Text},
};
use stm32h7xx_hal::Never;

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

pub const ORANGE: Rgb565 = Rgb565::new(255, 165, 0);
pub const MARGIN_X: u32 = 10;
pub const MARGIN_Y: u32 = 10;

fn rectangle_style(item_n: usize, cur_item: usize) -> PrimitiveStyle<Rgb565> {
  let mut style = PrimitiveStyleBuilder::new()
    .stroke_width(1)
    .stroke_color(Rgb565::BLACK);

  if item_n == cur_item {
    style = style.fill_color(ORANGE);
  }

  style.build()
}

pub fn draw_button(
  display: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  state: &Model,
  top_left: &Point,
  button_i: usize,
  text: &str,
) -> Result<Point, ViewError> {
  let fonts = get_fonts();
  let width: u32 = display.bounding_box().bottom_right().unwrap().x as u32 - MARGIN_X * 2;
  let corner_radius = Size::new(10, 10);
  let rectangle_size = Size::new(width, 30);

  let rec =
    RoundedRectangle::with_equal_corners(Rectangle::new(*top_left, rectangle_size), corner_radius)
      .into_styled(rectangle_style(button_i, state.selected_item));
  rec.draw(display)?;

  // Draw centered text.
  Text::with_alignment(
    text,
    rec.bounding_box().center() + Point::new(0, 5),
    if button_i == state.selected_item {
      fonts.white
    } else {
      fonts.black
    },
    Alignment::Center,
  )
  .draw(display)?;

  return Ok(Point::new(
    MARGIN_X as i32,
    top_left.y + (rectangle_size.height + MARGIN_Y) as i32,
  ));
}

pub fn draw_nav(
  display: &mut impl DrawTarget<Color = Rgb565, Error = ViewError>,
  text: &str,
) -> Result<Point, ViewError> {
  let fonts = get_fonts();

  let nav_bar = Rectangle::new(
    Point::new(0, 0),
    Size::new(display.bounding_box().size.width, 30),
  );
  nav_bar
    .into_styled(PrimitiveStyle::with_fill(ORANGE))
    .draw(display)?;
  Line::new(
    Point::new(0, 30),
    display.bounding_box().bottom_right().unwrap().x_axis() + Point::new(0, 30),
  )
  .into_styled(PrimitiveStyle::with_stroke(ViewColor::BLACK, 1))
  .draw(display)?;

  Text::with_alignment(
    text,
    nav_bar.bounding_box().center() + Point::new(0, 5),
    fonts.white,
    Alignment::Center,
  )
  .draw(display)?;

  return Ok(Point::new(
    0,
    nav_bar.bounding_box().bottom_right().unwrap().y,
  ));
}
