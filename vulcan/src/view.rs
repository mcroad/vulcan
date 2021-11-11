use crate::{
  home::Home,
  splash::Splash,
  types::{Page, PageViewResult, Screen, State, ViewColor, ViewError},
};
use embedded_graphics::draw_target::DrawTarget;

pub fn view(
  display: &mut impl DrawTarget<Color = ViewColor, Error = ViewError>,
  state: &State,
) -> PageViewResult {
  match state.screen {
    Screen::Splash => Splash::view(display, state)?,
    Screen::Home => Home::view(display, state)?,
  }

  return Ok(());
}
