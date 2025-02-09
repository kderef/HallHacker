use crate::{Assets, State};
use macroquad::{prelude::*, ui::Skin};

use macroquad::ui::{hash, root_ui, widgets, Ui};
use std::ops::DerefMut;

pub const FONT_SCALE: f32 = 0.05;

pub async fn skin(root: impl DerefMut<Target = Ui>, assets: &Assets) -> Skin {
    let screen_w = screen_width();
    let font_size = (screen_w * FONT_SCALE) as u16;

    let base = || root
        .style_builder()
        .with_font(&assets.font)
        .unwrap();

    let label_style = base()
        .text_color(GREEN)
        .font_size(font_size)
        .build();

    let window_style = base()
        .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
        .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
        .build();

    let button_style = base()
        .text_color(GREEN)
        .text_color_hovered(DARKGREEN)
        .color(Color::new(0.15, 0.15, 0.15, 1.0))
        .font_size(font_size)
        .build();

    let editbox_style = base()
        .background_margin(RectOffset::new(0., 0., 0., 0.))
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .color_selected(Color::from_rgba(190, 190, 190, 255))
        .font_size(font_size)
        .build();

    Skin {
        editbox_style,
        window_style,
        button_style,
        label_style,
        ..root.default_skin()
    }
}

pub fn button(text: &str, bounds: Rect) -> bool {
    let Rect { x, y, w, h } = bounds;
    draw_rectangle(x, y, w, h, GRAY);

    // Text pos
    let font_size = w * 0.1;
    let text_size = measure_text(text, None, font_size as u16, 1.0);

    draw_text(text, x, y, font_size, BLACK);

    // Clicked
    let mouse_pos: Vec2 = mouse_position().into();
    if bounds.contains(mouse_pos) {
        is_mouse_button_pressed(MouseButton::Left)
    } else {
        false
    }
}

/// draw the main menu and return state change
pub async fn draw_main_menu(assets: &Assets) -> Option<State> {
    clear_background(BLACK);

    // data
    let (screen_w, screen_h) = (screen_width(), screen_height());
    let center_x = screen_w / 2.0;

    // title text
    let title_text = "HallHacker";
    let title_text_size = (screen_w * 0.1) as u16;
    let title_font = None;
    let title_actual_size = measure_text(title_text, title_font, title_text_size, 1.0);

    let title_y = title_text_size as f32;

    draw_text(
        title_text,
        center_x - title_actual_size.width / 2.,
        title_y,
        title_text_size as f32,
        WHITE,
    );

    // Draw the actual buttons
    button("Play", Rect::new(20.0, 50.0, 100.0, 40.0));

    root_ui().button(vec2(1.0, 1.0), "PLAY");

    None
}
