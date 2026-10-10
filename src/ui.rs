use macroquad::prelude::*;
use macroquad::ui::{Skin, hash, root_ui, widgets};

const FONT_SIZE: f32 = 32.;

pub struct State {
    pub expression: String,
}

impl State {
    pub fn new() -> State {
        State {
            expression: String::new(),
        }
    }
}

/// Draws the UI with whatever `state` given.
pub fn draw(state: &mut State) {
    widgets::Window::new(hash!(), vec2(20., 220.), vec2(320., 400.))
        .label("Input menu")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.input_text(hash!(), "Expression", &mut state.expression);
        });
}

/// Pushes the custom UI styles onto `root_ui`.
/// # Usage
/// Call **once** at the start of the program (outside the main loop).
pub fn push_styles() {
    let skin = skin();
    root_ui().push_skin(&skin);
}

fn skin() -> Skin {
    let window_titlebar_style = root_ui()
        .style_builder()
        .font_size(FONT_SIZE as u16)
        .text_color(WHITE)
        .build();
    let label_style = root_ui()
        .style_builder()
        .font_size(FONT_SIZE as u16)
        .text_color(WHITE)
        .build();
    let editbox_style = root_ui()
        .style_builder()
        .font_size(FONT_SIZE as u16)
        .build();
    let window_style = root_ui()
        .style_builder()
        .font_size(FONT_SIZE as u16)
        .background(Image::gen_image_color(320, 400, DARKBLUE))
        .build();
    Skin {
        window_style,
        window_titlebar_style,
        label_style,
        editbox_style,
        title_height: FONT_SIZE,
        ..root_ui().default_skin()
    }
}

pub fn draw_debug_info(camera: &Camera3D, delta: f32, theta: f32) {
    let debug_info = format!(
        "\
            [camera] \n\
            * direction: ({}, {}, {}) \n\
            * position: ({}, {}, {}) \n\n\
            [general] \n\
            * fps: {} \n\
            * delta: {} \n\
            * theta: {}",
        camera.target.x,
        camera.target.y,
        camera.target.z,
        camera.position.x,
        camera.position.y,
        camera.position.z,
        get_fps(),
        delta,
        theta
    );
    draw_multiline_text(debug_info, 0., 20., FONT_SIZE, None, WHITE);
}
