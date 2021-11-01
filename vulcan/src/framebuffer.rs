use embedded_graphics::prelude::*;

const SIZE: usize = 240;

pub struct Framebuffer<C> {
  pixels: [[C; SIZE]; SIZE],
}

impl<C> Framebuffer<C>
where
  C: RgbColor,
{
  pub fn new() -> Self {
    Self {
      pixels: [[C::BLACK; 240]; 240],
    }
  }

  fn set_pixel(&mut self, position: Point, color: C) {
    let (x, y) = (position.x as usize, position.y as usize);
    self
      .pixels
      .get_mut(y)
      .and_then(|row| row.get_mut(x))
      .map(|pixel| *pixel = color);
  }
}

impl<C> DrawTarget for Framebuffer<C>
where
  C: RgbColor,
{
  type Color = C;
  type Error = core::convert::Infallible;

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

impl<C> OriginDimensions for Framebuffer<C> {
  fn size(&self) -> Size {
    Size::new_equal(SIZE as u32)
  }
}

impl<C> Drawable for Framebuffer<C>
where
  C: RgbColor,
{
  type Color = C;
  type Output = Point;
  fn draw<D>(&self, target: &mut D) -> Result<Point, D::Error>
  where
    D: DrawTarget<Color = Self::Color>,
  {
    let mut buff = [C::BLACK; SIZE * SIZE];
    for (x, row) in self.pixels.iter().enumerate() {
      for (y, color) in row.iter().enumerate() {
        buff[x * SIZE + y] = *color;
      }
    }
    target.fill_contiguous(&self.bounding_box(), buff)?;

    return Ok(Point { x: 0, y: 0 });
  }
}
