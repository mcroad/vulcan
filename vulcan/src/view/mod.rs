use self::{
  create::create_wallet,
  export::export_wallet,
  home::home,
  sign::sign_transaction,
  splash::splash,
  util::{ViewColor, ViewError, ViewResult},
};
use crate::types::{Model, Screen};
use embedded_graphics::draw_target::DrawTarget;

mod create;
mod export;
mod home;
mod sign;
mod splash;
pub mod util;

pub fn view(
  display: &mut impl DrawTarget<Color = ViewColor, Error = ViewError>,
  state: &Model,
) -> ViewResult {
  return match state.screen {
    Screen::Splash => splash(display, state),
    Screen::Home => home(display, state),
    Screen::Create => create_wallet(display, state),
    Screen::ExportWallet(export_screen) => export_wallet(display, state, export_screen),
    Screen::Sign => sign_transaction(display, state),
  };
}
