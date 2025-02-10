use crate::{screen_size, Assets, SettingsPage, State};
use macroquad::{prelude::*, ui::Skin};

use crate::Origin;
use macroquad::ui::{hash, root_ui, widgets, Ui};

pub const FONT_SCALE: f32 = 0.035;

pub async fn skin(assets: &Assets) -> Skin {
    let screen_w = screen_width();
    let font_size = (screen_w * FONT_SCALE) as u16;
    let root = root_ui();

    let base = || {
        root.style_builder()
            .font_size(font_size)
            .with_font(&assets.font)
            .unwrap()
    };

    let label_style = base().text_color(WHITE).font_size(font_size).build();

    let window_style = base()
        .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
        .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
        .build();

    const BTN_COLOR: Color = Color::new(0.15, 0.15, 0.15, 1.0);

    let button_style = base()
        .text_color(GREEN)
        .text_color_hovered(DARKGREEN)
        .color(BTN_COLOR)
        .margin(RectOffset::new(10.0, 10.0, 10.0, 10.0))
        .text_color_hovered(YELLOW)
        .color_hovered(BTN_COLOR)
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

pub fn draw_text_centered<S: AsRef<str>>(
    text: S,
    font: Option<&Font>,
    pos: Vec2,
    size: f32,
    color: Color,
) -> Vec2 {
    let text = text.as_ref();
    let text_size = {
        let s = measure_text(text, font, size as u16, 1.0);
        vec2(s.width, s.height)
    };

    let npos = pos - text_size / 2.0;

    draw_text(text, npos.x, npos.y, size, color);
    text_size
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

fn main_menu_bg(assets: &Assets) {
    clear_background(BLACK);

    let size = screen_size();

    let font_size = 10;
    let char_size = measure_text("0", Some(&assets.font), font_size, 1.0);

    for y in 0..(size.y / char_size.height) as i32 {
        for x in 0..(size.x / char_size.width) as i32 {
            //
        }
    }
}

/// draw the main menu and return state change
pub async fn draw_main_menu(assets: &Assets) -> Option<State> {
    main_menu_bg(assets);

    let skin = skin(assets).await;
    root_ui().pop_skin();
    root_ui().push_skin(&skin);

    // data
    let screen_size = screen_size();
    let center = screen_size / 2.0;

    // title text
    let title = "HallHacker";
    let title_size = screen_size.x * 0.1;
    let title_font = None;

    let start_y = center.y - title_size;

    let text_size = draw_text_centered(
        title,
        title_font,
        vec2(center.x, start_y),
        title_size,
        WHITE,
    );

    let mut button_pos = vec2(center.x - text_size.x / 2.0, start_y);
    let spacing = text_size.y + 10.0;

    // Draw the actual buttons
    if root_ui().button(button_pos, "  PLAY  ") {
        return Some(State::Playing);
    }
    button_pos.y += spacing;

    if root_ui().button(button_pos, "SETTINGS") {
        return Some(State::Settings(Origin::Menu, SettingsPage::Video));
    }

    None
}

pub async fn draw_pause_menu(assets: &Assets) -> Option<State> {
    None
}
