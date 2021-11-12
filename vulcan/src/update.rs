use crate::{
  keypad::Key,
  types::{Cmd, KeypadMode, Model, Msg, Screen},
};

pub fn update(state: &mut Model, msg: Msg) -> Cmd {
  match msg {
    Msg::Navigate(screen) => {
      state.screen = screen;

      match screen {
        Screen::Splash => {
          return Cmd::UpdateAfter(4000, Msg::Navigate(Screen::Home));
        }
        _ => {}
      }
    }
    Msg::KeyUp(key) => match state.keypad_mode {
      KeypadMode::Text => match key {
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
      KeypadMode::Navigation => match key {
        Key::Up => {
          if state.selected_item > 0 {
            state.selected_item = state.selected_item - 1;
          }
        }
        Key::Down => {
          if state.selected_item < 4 {
            state.selected_item = state.selected_item + 1;
          }
        }
        _ => {}
      },
      _ => {}
    },
  }

  Cmd::None
}
