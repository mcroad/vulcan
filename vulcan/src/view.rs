use crate::types::{Display, Page, PageViewResult, Screen, State};
use crate::{home::Home, splash::Splash};

pub fn view(display: &mut Display, state: &State) -> PageViewResult {
  match state.screen {
    Screen::Splash => Splash::view(display, state)?,
    Screen::Home => Home::view(display, state)?,
  }

  return Ok(());
}
