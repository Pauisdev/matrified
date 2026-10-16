use crate::state::State;
use egui_macroquad::egui;
use macroquad::prelude::*;

const FONT_SIZE: f32 = 32.;

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
            [general] \n\
            * fps: {} \n\
            ",
        state.position,
        get_fps(),
    );
    draw_multiline_text(debug_info, 0., 20., FONT_SIZE, None, WHITE);
}
