use crate::view::util::{ViewColor, ViewError};
use embedded_graphics::prelude::*;

const WIDTH: usize = 240;
const HEIGHT: usize = 320;
const SIZE: usize = WIDTH * HEIGHT;

pub struct Framebuffer {
  buffer: [ViewColor; SIZE],
  pixels: [ViewColor; SIZE],
}

impl Framebuffer {
  pub fn new() -> Self {
    Self {
      // what the user draws to
      buffer: [ViewColor::BLACK; SIZE],
      // keeps track of the current state of the screen
      pixels: [ViewColor::BLACK; SIZE],
    }
  }

  pub fn draw<D>(&mut self, target: &mut D) -> Result<(), D::Error>
  where
    D: DrawTarget<Color = ViewColor>,
  {
    // colors must impl Iterator<Item = ViewColor>
    let buffer = self.buffer.iter().map(|color| *color);

    // no diffing. re-draw the whole screen
    // target.fill_contiguous(&self.bounding_box(), buffer)?;

    // only draw pixels that change
    let diff = buffer
      .enumerate()
      .filter(|(i, color)| self.pixels[*i] != *color)
      .map(|(i, color)| {
        let y = i / WIDTH;
        let x = i - (y * WIDTH);
        Pixel(Point::new(x as i32, y as i32), color)
      });
    target.draw_iter(diff)?;

    // the buffer is the new screen state
    self.pixels = self.buffer;

    return Ok(());
  }

  fn set_pixel(&mut self, position: Point, color: ViewColor) {
    let (x, y) = (position.x as usize, position.y as usize);

    // renders mirrored screen. not sure why
    // let index = x * WIDTH + y;
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
