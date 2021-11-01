use crate::types::{Display, Page, Screen, State};
use crate::{home::Home, splash::Splash};

pub fn view(display: &mut Display, state: &State) {
  match state.screen {
    Screen::Splash => Splash::view(display, state).unwrap(),
    Screen::Home => Home::view(display, state).unwrap(),
  }
}
