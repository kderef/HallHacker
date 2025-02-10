//! Application level code

mod app;
mod assets;
mod settings;
mod state;
mod system;
pub mod ui;

pub use app::*;
pub use assets::*;
pub use settings::*;
pub use state::*;

pub use ::rand;
pub use bincode;
pub use macroquad;
pub use serde;

use macroquad::prelude::*;

// some utility functions

pub fn screen_size() -> Vec2 {
    vec2(screen_width(), screen_height())
}
