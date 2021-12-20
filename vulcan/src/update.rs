use crate::{
  keypad::Key,
  types::{Cmd, ExportScreen, KeypadMode, Model, Msg, Screen},
};

fn go_up(state: &mut Model) {
  if state.selected_item > 0 {
    state.selected_item = state.selected_item - 1;
  }
}
fn go_down(state: &mut Model, max: usize) {
  if state.selected_item < max {
    state.selected_item = state.selected_item + 1;
  }
}

pub fn update(state: &mut Model, msg: Msg) -> Cmd {
  if state.screen == Screen::Splash {
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
          Key::Up => go_up(state),
          Key::Down => go_down(state, 4),
          _ => {}
        },
        _ => {}
      },
    }
  } else if state.screen == Screen::Home {
    match msg {
      Msg::KeyUp(key) => match state.keypad_mode {
        KeypadMode::Navigation => match key {
          Key::Up => go_up(state),
          Key::Down => go_down(state, state.home_menu.len()),
          Key::Forward => {
            match state.selected_item {
              0 => {
                state.screen = Screen::Create;
              }
              1 => {
                state.screen = Screen::Sign;
              }
              2 => {
                state.screen = Screen::ExportWallet(ExportScreen::Menu);
              }
              _ => {}
            }
            state.selected_item = 0;
          }
          _ => {}
        },
        _ => {}
      },
      _ => {}
    }
  } else if state.screen == Screen::Create {
    match msg {
      Msg::KeyUp(key) => match state.keypad_mode {
        KeypadMode::Navigation => match key {
          Key::Back => {
            state.screen = Screen::Home;
            state.selected_item = 0;
          }
          _ => {}
        },
        _ => {}
      },
      _ => {}
    }
  } else if state.screen == Screen::Sign {
    match msg {
      Msg::KeyUp(key) => match state.keypad_mode {
        KeypadMode::Navigation => match key {
          Key::Back => {
            state.screen = Screen::Home;
            state.selected_item = 0;
          }
          _ => {}
        },
        _ => {}
      },
      _ => {}
    }
  } else if state.screen == Screen::ExportWallet(ExportScreen::Menu) {
    match msg {
      Msg::KeyUp(key) => match state.keypad_mode {
        KeypadMode::Navigation => match key {
          Key::Up => go_up(state),
          Key::Down => go_down(state, 2),
          Key::Back => {
            state.screen = Screen::Home;
            state.selected_item = 0;
          }
          _ => {}
        },
        _ => {}
      },
      _ => {}
    }
  }

  Cmd::None
}
