use crate::types::{Display, Screen, Model};
use embedded_graphics::draw_target::DrawTarget;
use home::home;
use splash::splash;
use stm32h7xx_hal::Never;

mod home;
mod splash;

pub type ViewColor = <Display as DrawTarget>::Color;
pub type ViewError = st7789::Error<Never>;
pub type ViewResult = Result<(), ViewError>;

pub fn view(
  display: &mut impl DrawTarget<Color = ViewColor, Error = ViewError>,
  state: &Model,
) -> ViewResult {
  return match state.screen {
    Screen::Splash => splash(display, state),
    Screen::Home => home(display, state),
  };
}
