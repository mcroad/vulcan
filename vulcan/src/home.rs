use crate::types::{Display, Page, PageViewResult, State};
use embedded_graphics::{
  mono_font::{ascii::FONT_10X20, MonoTextStyle},
  pixelcolor::Rgb565,
  prelude::*,
  primitives::{PrimitiveStyleBuilder, StrokeAlignment},
  text::{Alignment, Text},
};

pub struct Home {}
impl Page for Home {
  fn view(display: &mut Display, state: &State) -> PageViewResult {
    // Create styles used by the drawing operations.
    let border_stroke = PrimitiveStyleBuilder::new()
      .stroke_color(Rgb565::CYAN)
      .stroke_width(10)
      .stroke_alignment(StrokeAlignment::Inside)
      .build();
    let character_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CYAN);

    // Draw a 3px wide outline around the display.
    display
      .bounding_box()
      .into_styled(border_stroke)
      .draw(display)?;

    // Draw centered text.
    Text::with_alignment(
      &state.msg,
      display.bounding_box().center() + Point::new(0, 15),
      character_style,
      Alignment::Center,
    )
    .draw(display)?;

    return Ok(());
  }
}
