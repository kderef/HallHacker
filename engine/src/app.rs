use macroquad::prelude::*;
pub use macroquad::prelude::Conf;

use crate::Settings;

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
        if !self.fullscreen { // fix window size
            self.reset_screen_size();
        }
    }
    pub fn reset_screen_size(&mut self) {
        request_new_screen_size(self.init_size.0 as f32, self.init_size.1 as f32);
    }

    pub fn check_system_keys(&mut self) {
        let keys = get_keys_pressed();

        for key in keys {
            match key {
                KeyCode::F11 => {
                    self.toggle_fullscreen();
                }
                _ => {}
            }
        }
    }
}
