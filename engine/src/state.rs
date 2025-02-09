use macroquad::prelude::*;

pub enum SettingsPage {
    Video,
    Audio,
    Controls,
}

pub enum State {
    Menu,
    Paused,
    Settings(SettingsPage),
    Playing,
}

impl Default for State {
    fn default() -> Self {
        Self::Menu
    }
}