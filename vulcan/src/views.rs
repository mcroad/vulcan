use embedded_graphics::{
    image::{Image, ImageRawLE},
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, StrokeAlignment},
    text::{Alignment, Text},
};

use crate::types::Display;

pub fn splash_screen(display: &mut Display) {
    let white_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);

    Text::with_alignment(
        "Vulcan",
        display.bounding_box().center() - Point::new(0, 70),
        white_style,
        Alignment::Center,
    )
    .draw(display)
    .unwrap();

    let raw_image_data = ImageRawLE::new(include_bytes!("../assets/ferris.raw"), 86);
    let volcano = Image::new(
        &raw_image_data,
        display.bounding_box().center() - Point::new(43, 40),
    );
    volcano.draw(display).unwrap();

    Text::with_alignment(
        concat!("Version\n", env!("GIT_HASH")),
        display.bounding_box().center() + Point::new(0, 60),
        white_style,
        Alignment::Center,
    )
    .draw(display)
    .unwrap();
}

pub fn home(display: &mut Display) {
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
        .draw(display)
        .unwrap();

    // Draw centered text.
    Text::with_alignment(
        "home screen",
        display.bounding_box().center() + Point::new(0, 15),
        character_style,
        Alignment::Center,
    )
    .draw(display)
    .unwrap();
}
