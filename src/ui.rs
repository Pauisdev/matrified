use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::math::SphericalCoordinates;

const FONT_SIZE: f32 = 32.;

pub struct State {
    pub position: SphericalCoordinates,
    pub zoom: f32,
    pub input_mode_enabled: bool,
    pub show_debug_info: bool,
    pub expression: String,
    pub render_distance: i32,
    pub root_splits: i32,
    pub root_find_tries: i32,
    pub sample_size: i32,
    pub distance_draw_cutoff: f32,
}

impl State {
    pub fn new() -> State {
        State {
            position: SphericalCoordinates::from_distance(10.),
            zoom: 0.,
            input_mode_enabled: false,
            show_debug_info: true,
            expression: String::new(),
            render_distance: 100,
            root_splits: 100,
            root_find_tries: 10,
            sample_size: 1000,
            distance_draw_cutoff: 40.,
        }
    }
}

/// Draws the UI with whatever `state` given.
pub fn draw(state: &mut State) {
    egui_macroquad::ui(|egui_ctx| {
        if !state.input_mode_enabled {
            return;
        }
        egui::Window::new("Options").show(egui_ctx, |ui| {
            ui.label("Expression");
            ui.text_edit_singleline(&mut state.expression);
            ui.separator();
            ui.label("Renderer");
            ui.add(
                egui::Slider::new(&mut state.render_distance, 0..=10000).text("Render distance"),
            );
            ui.add(egui::Slider::new(&mut state.sample_size, 0..=5000).text("Sample size"));
            ui.add(
                egui::Slider::new(&mut state.distance_draw_cutoff, 0.0..=200.0)
                    .text("Distance draw cutoff"),
            );
            ui.separator();
            ui.label("Root finder");
            ui.add(egui::Slider::new(&mut state.root_splits, 0..=500).text("Splits"));
            ui.add(egui::Slider::new(&mut state.root_find_tries, 1..=100).text("Tries"));
        });
    });
}

pub fn draw_debug_info(state: &State) {
    let debug_info = format!(
        "\
            [camera] \n\
            * position: {} \n\
            * zoom: {:.2}\n\n\
            [general] \n\
            * fps: {} \n\
            ",
        state.position,
        state.zoom,
        get_fps(),
    );
    draw_multiline_text(debug_info, 0., 20., FONT_SIZE, None, WHITE);
}
