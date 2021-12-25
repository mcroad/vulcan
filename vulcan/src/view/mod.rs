use self::{
  create::create_wallet,
  export::export_wallet,
  home::home,
  sign::sign_transaction,
  splash::splash,
  util::{ViewColor, ViewError, ViewResult},
  verify::verify_address,
};
use crate::types::{Model, Screen};
use embedded_graphics::draw_target::DrawTarget;

mod create;
mod export;
mod home;
mod sign;
mod splash;
pub mod util;
mod verify;

pub fn view(
  display: &mut impl DrawTarget<Color = ViewColor, Error = ViewError>,
  state: &Model,
) -> ViewResult {
  return match &state.screen {
    Screen::Splash => splash(display, state),
    Screen::Home => home(display, state),
    Screen::Create => create_wallet(display, state),
    Screen::Verify => verify_address(display, state),
    Screen::ExportWallet(screen) => export_wallet(display, state, screen),
    Screen::Sign(screen) => sign_transaction(display, state, screen),
  };
}
