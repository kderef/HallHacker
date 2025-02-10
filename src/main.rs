#![allow(non_snake_case)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use engine::{
    macroquad::{self, ui::root_ui},
    rand, ui, Assets, Settings, State,
};
use game::Game;

use engine::{app, App};
use macroquad::prelude::*;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERS: &str = env!("CARGO_PKG_VERSION");

#[macroquad::main(app)]
async fn main() {
    println!("Starting {PKG_NAME} version {PKG_VERS}");

    // Settings
    let settings;
    if !Settings::file_exists() {
        settings = Settings::default();
        println!("No settings found, using default: {settings:#?}");
        settings.write().unwrap();
    } else {
        settings = Settings::read().unwrap();
        println!("Read settings: {settings:#?}")
    }

    let app = App::new();
    let mut game = Game::new(app, &settings);

    // Assets
    let mut assets = Assets::load().await;

    // UI skin
    let skin = engine::ui::skin(&assets).await;
    root_ui().push_skin(&skin);

    loop {
        game.app.check_system_keys(&mut game.state);

        match game.state {
            State::Menu => {
                if let Some(change) = ui::draw_main_menu(&assets).await {
                    game.state = change;
                }
            }
            State::Playing => {
                game.handle_inputs();
                game.draw();
            }
            State::Paused => {
                game.draw();
                if let Some(change) = ui::draw_pause_menu(&assets).await {
                    game.state = change;
                }
            }
            State::Settings(origin, page) => {}
            _ => unimplemented!(),
        }

        next_frame().await;
    }
}
