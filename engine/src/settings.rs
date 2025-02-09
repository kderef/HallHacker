use std::{fs, path::{self, Path, PathBuf}};

use macroquad::prelude::*;
use serde::{Serialize, Deserialize};

pub const SETTINGS_FILE: &str = "settings.bin";


#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub fov: f32,
    pub sample_count: i32,
    pub high_dpi: bool,
    pub start_fullscreen: bool,
}

impl Settings {
    pub fn read() -> anyhow::Result<Self> {
        let bytes = fs::read(SETTINGS_FILE)?;
        let s = bincode::deserialize(&bytes)?;
        Ok(s)
    }
    pub fn write(&self) -> anyhow::Result<()> {
        fs::write(SETTINGS_FILE, bincode::serialize(self)?)?;
        Ok(())
    }

    pub fn file_exists() -> bool {
        Path::new(SETTINGS_FILE)
            .is_file()
    }
}

impl Default for Settings {
    fn default() -> Self {
        let c = Conf::default();
    
        Self {
            fov: 90.0,
            sample_count: c.sample_count,
            high_dpi: true,
            start_fullscreen: false,
        }
    }
}
impl Into<Conf> for Settings {
    fn into(self) -> Conf {
        Conf {
            fullscreen: self.start_fullscreen,
            window_title: "HallHacker".to_owned(),
            window_width: 950,
            window_height: 500,
            high_dpi: self.high_dpi,
            sample_count: self.sample_count,
            window_resizable: false,
            icon: None,
            ..Default::default()
        }
    }
}