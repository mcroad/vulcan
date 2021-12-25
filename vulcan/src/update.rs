use crate::{
  keypad::{Key, NavigationKey},
  types::{Cmd, ExportScreen, KeyType, Model, Msg, Screen},
};

fn go_up(state: &mut Model) {
  if state.selected_item > 0 {
    state.selected_item = state.selected_item - 1;
  }
}
fn go_down(state: &mut Model, max: usize) {
  if state.selected_item < (max - 1) {
    state.selected_item = state.selected_item + 1;
  }
}
fn go_home(state: &mut Model) {
  state.screen = Screen::Home;
  state.selected_item = 0;
}

fn splash(state: &mut Model, msg: Msg) -> Cmd {
  match msg {
    Msg::Navigate(screen) => {
      state.screen = screen;

      match screen {
        Screen::Splash => {
          return Cmd::UpdateAfter(2000, Msg::Navigate(Screen::Home));
        }
        _ => {}
      }
    }
    Msg::KeyUp(key_type) => match key_type {
      KeyType::Text(key) => match key {
        Key::Back => {
          let len = state.msg.len();
          if len > 0 {
            state.msg.truncate(len - 1);
          }
        }
        Key::Forward => {}
        _ => {
          state.msg.push_str(key.to_string()).ok();
          defmt::info!("key {}", key);
        }
      },
      KeyType::Navigation(key) => match key {
        NavigationKey::Up => go_up(state),
        NavigationKey::Down => go_down(state, 4),
        _ => {}
      },
      _ => {}
    },
  }

  return Cmd::None;
}

pub fn update(state: &mut Model, msg: Msg) -> Cmd {
  match state.screen {
    Screen::Splash => {
      let cmd = splash(state, msg);
      if cmd != Cmd::None {
        return cmd;
      }
    }
    Screen::Home => match msg {
      Msg::KeyUp(key_type) => match key_type {
        KeyType::Navigation(key) => match key {
          NavigationKey::Up => go_up(state),
          NavigationKey::Down => go_down(state, state.home_menu.len()),
          NavigationKey::Forward => {
            match state.selected_item {
              0 => {
                state.screen = Screen::Create;
              }
              1 => {
                state.screen = Screen::Sign;
              }
              2 => {
                state.screen = Screen::Verify;
              }
              3 => {
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
    },
    Screen::Create => match msg {
      Msg::KeyUp(key_type) => match key_type {
        KeyType::Navigation(key) => match key {
          NavigationKey::Back => go_home(state),
          _ => {}
        },
        _ => {}
      },
      _ => {}
    },
    Screen::Sign => match msg {
      Msg::KeyUp(key_type) => match key_type {
        KeyType::Navigation(key) => match key {
          NavigationKey::Back => go_home(state),
          _ => {}
        },
        _ => {}
      },
      _ => {}
    },
    Screen::Verify => match msg {
      Msg::KeyUp(key_type) => match key_type {
        KeyType::Navigation(key) => match key {
          NavigationKey::Back => go_home(state),
          _ => {}
        },
        _ => {}
      },
      _ => {}
    },
    Screen::ExportWallet(screen) => match screen {
      ExportScreen::Menu => match msg {
        Msg::KeyUp(key_type) => match key_type {
          KeyType::Navigation(key) => match key {
            NavigationKey::Up => go_up(state),
            NavigationKey::Down => go_down(state, 3),
            NavigationKey::Back => go_home(state),
            NavigationKey::Forward => {
              match state.selected_item {
                0 => {
                  state.screen = Screen::ExportWallet(ExportScreen::SeedQR);
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
      },
      ExportScreen::SeedQR => match msg {
        Msg::KeyUp(key_type) => match key_type {
          KeyType::Navigation(key) => match key {
            NavigationKey::Back => {
              state.screen = Screen::ExportWallet(ExportScreen::Menu);
              state.selected_item = 0;
            }
            _ => {}
          },
          _ => {}
        },
        _ => {}
      },
      _ => {}
    },
  }

  Cmd::None
}
