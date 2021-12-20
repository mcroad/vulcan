use crate::view::util::{ViewColor, ViewError};
use embedded_graphics::prelude::*;

const WIDTH: usize = 240;
const HEIGHT: usize = 320;

pub struct Framebuffer {
  buffer: [ViewColor; WIDTH * HEIGHT],
  // pixels: [ViewColor; WIDTH * HEIGHT],
}

impl Framebuffer {
  pub fn new() -> Self {
    Self {
      // what the user draws to
      buffer: [ViewColor::BLACK; WIDTH * HEIGHT],
      // keeps track of the current state of the screen
      // pixels: [ViewColor::BLACK; WIDTH * HEIGHT],
    }
  }

  fn set_pixel(&mut self, position: Point, color: ViewColor) {
    let (x, y) = (position.x as usize, position.y as usize);

    // renders mirrored screen. not sure why
    // let index = x * SIZE + y;
    // renders correct screen
    let index = y * WIDTH + x;

    self.buffer[index] = color;
  }
}

impl DrawTarget for Framebuffer {
  type Color = ViewColor;
  type Error = ViewError;

  fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
  where
    I: IntoIterator<Item = Pixel<Self::Color>>,
  {
    for Pixel(position, color) in pixels {
      self.set_pixel(position, color);
    }

    Ok(())
  }
}

impl OriginDimensions for Framebuffer {
  fn size(&self) -> Size {
    Size::new(WIDTH as u32, HEIGHT as u32)
  }
}

impl Drawable for Framebuffer {
  type Color = ViewColor;
  type Output = Point;
  fn draw<D>(&self, target: &mut D) -> Result<Point, D::Error>
  where
    D: DrawTarget<Color = Self::Color>,
  {
    // colors must impl Iterator<Item = ViewColor>
    let colors = self.buffer.iter().map(|color| *color);
    // for testing purposes
    // let colors = core::iter::repeat(ViewColor::MAGENTA);

    target.fill_contiguous(&self.bounding_box(), colors)?;

    // out of memory or possible flip-link bug
    // let pixels = self.pixels.iter().map(|color| *color);
    // let diff = colors
    //   .enumerate()
    //   .filter(|(i, color)| pixels[i] != color)
    //   .map(|(i, color)| {
    //     let y = i / SIZE;
    //     let x = i - y;
    //     Pixel(Point::new(x, y), color)
    //   });
    // target.draw_iter(diff)?;

    return Ok(Point::new(WIDTH as i32, HEIGHT as i32));
  }
}
