use engine::App;
use engine::macroquad;
use engine::Settings;
use engine::State;
use macroquad::prelude::*;


pub struct Game {
    pub app: App,
    pub state: State,

    camera: Camera3D,
}

impl Game {
    pub fn new(app: App, settings: &Settings) -> Self {
        Self {
            app,
            camera: Camera3D {
                fovy: settings.fov,
                position: vec3(1.0, 1.0, 1.0),
                target: Vec3::ZERO,
                up: vec3(0.0, 1.0, 0.0),
                aspect: None,
                projection: Projection::Perspective,
                render_target: None,
                viewport: None,
            },
            state: State::default(),
        }
    }
    pub fn handle_inputs(&mut self) {
        // TODO
    }
    pub fn draw(&mut self) {
        set_camera(&self.camera);

        draw_grid(100, 1.0, WHITE, YELLOW);

        set_default_camera();

        draw_text(&format!("FPS: {}", get_fps()), 0.0, 20.0, 20.0, PURPLE);
    }
}