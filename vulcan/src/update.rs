use crate::{
  keypad::Key,
  types::{Cmd, Msg, State},
};

pub fn update(state: &mut State, msg: Msg) -> Cmd {
  match msg {
    Msg::Navigate(screen) => {
      state.screen = screen;
    }
    Msg::KeyUp(key) => match key {
      Key::Back => {
        let len = state.msg.len();
        if len > 0 {
          state.msg.truncate(len - 1);
        }
      }
      Key::Forward => {}
      Key::Zero => {
        defmt::info!("zero");
      }
      _ => {
        state.msg.push_str(key.to_string()).ok();
        defmt::info!("key {}", key);
      }
    },
  }

  Cmd::Noop
}
