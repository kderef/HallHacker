pub use macroquad::prelude::Conf;
use macroquad::prelude::*;

use crate::{Origin, Settings, State};

pub fn app() -> Conf {
    Settings::read().unwrap_or_default().into()
}

pub struct App {
    fullscreen: bool,
    init_size: (i32, i32),
}

impl App {
    pub fn new() -> Self {
        let c = app();
        Self {
            fullscreen: c.fullscreen,
            init_size: (c.window_width, c.window_height),
        }
    }
    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen ^= true;
        set_fullscreen(self.fullscreen);
        if !self.fullscreen {
            // fix window size
            self.reset_screen_size();
        }
    }
    pub fn reset_screen_size(&mut self) {
        request_new_screen_size(self.init_size.0 as f32, self.init_size.1 as f32);
    }

    pub fn check_system_keys(&mut self, prev_state: &mut State) {
        let keys = get_keys_pressed();
        let mut new_state = *prev_state;

        for key in keys {
            match key {
                KeyCode::F11 => {
                    self.toggle_fullscreen();
                }
                KeyCode::Escape => {
                    new_state = match *prev_state {
                        State::Settings(Origin::Menu, _) => State::Menu,
                        State::Settings(Origin::Game, _) => State::Playing,
                        State::Playing => State::Paused,
                        State::Paused => State::Playing,
                        s => s,
                    };
                }
                _ => {}
            }
        }

        *prev_state = new_state;
    }
}
