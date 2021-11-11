use crate::view::{ViewColor, ViewError};
use embedded_graphics::prelude::*;

const SIZE: usize = 240;

pub struct Framebuffer {
  pub pixels: [ViewColor; SIZE * SIZE],
}

impl Framebuffer {
  pub fn new() -> Self {
    Self {
      pixels: [ViewColor::BLACK; SIZE * SIZE],
    }
  }

  fn set_pixel(&mut self, position: Point, color: ViewColor) {
    let (x, y) = (position.x as usize, position.y as usize);

    // renders mirrored screen. not sure why
    // let index = x * SIZE + y;
    // renders correct screen
    let index = y * SIZE + x;

    self.pixels[index] = color;
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
    Size::new_equal(SIZE as u32)
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
    let colors = self.pixels.iter().map(|color| *color);
    // for testing purposes
    // let colors = core::iter::repeat(ViewColor::MAGENTA);

    target.fill_contiguous(&self.bounding_box(), colors)?;

    return Ok(Point::new_equal(SIZE as i32));
  }
}
