use std::ops::Range;

use macroquad::prelude::*;
use macroquad::ui::{Skin, hash, root_ui, widgets};

const FONT_SIZE: f32 = 32.;

pub struct State {
    pub rotation_radians: f32,
    pub zoom: f32,
    pub input_mode_enabled: bool,
    pub show_debug_info: bool,
    pub expression: String,
    pub render_range: Range<f32>,
}

impl State {
    pub fn new() -> State {
        State {
            rotation_radians: 0.,
            zoom: 0.,
            input_mode_enabled: false,
            show_debug_info: true,
            expression: String::new(),
            render_range: -100.0..100.0,
        }
    }
}

/// Draws the UI with whatever `state` given.
pub fn draw(state: &mut State) {
    widgets::Window::new(hash!(), vec2(20., 220.), vec2(500., 400.))
        .label("Input menu")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.input_text(hash!(), "Expression", &mut state.expression);
            ui.separator();
            ui.label(None, "Function");
            ui.drag(hash!(), "Min", None, &mut state.render_range.start);
            ui.drag(hash!(), "Max", None, &mut state.render_range.end);
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

pub fn draw_debug_info(camera: &Camera3D, state: &State) {
    let debug_info = format!(
        "\
            [camera] \n\
            * direction: ({}, {}, {}) \n\
            * position: ({:.2}, {:.2}, {:.2}) \n\
            * rotation_radians: {:.2}pi\n\
            * zoom: {:.2}\n\n\
            [general] \n\
            * fps: {} \n\
            ",
        camera.target.x,
        camera.target.y,
        camera.target.z,
        camera.position.x,
        camera.position.y,
        camera.position.z,
        state.rotation_radians,
        state.zoom,
        get_fps(),
    );
    draw_multiline_text(debug_info, 0., 20., FONT_SIZE, None, WHITE);
}
