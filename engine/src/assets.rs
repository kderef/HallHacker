use macroquad::prelude::*;

pub struct Assets {
    pub font: Font,
}

impl Assets {
    pub async fn load() -> Self {
        let mut font = load_ttf_font("assets/fonts/VCR_OSD_MONO_1.001.ttf")
            .await
            .unwrap();

        font.set_filter(FilterMode::Nearest);

        Self { font }
    }
}
