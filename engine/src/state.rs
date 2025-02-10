use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum Origin {
    Menu,
    Game,
}

#[derive(Clone, Copy)]
pub enum SettingsPage {
    GamePlay,
    Video,
    Audio,
    Controls,
}

#[derive(Clone, Copy)]
pub enum State {
    Menu,
    Paused,
    Settings(Origin, SettingsPage),
    Playing,
}

impl Default for State {
    fn default() -> Self {
        Self::Menu
    }
}
