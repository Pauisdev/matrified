mod function;
mod math;
mod ui;
use std::f32::consts::PI;

use macroquad::prelude::*;
use ui::State;

#[macroquad::main("Matrified")]
async fn main() {
    set_fullscreen(true);
    let mut state = State::new();
    let mut camera = Camera3D {
        up: vec3(0., 1., 0.),
        ..Default::default()
    };

    loop {
        clear_background(BLACK);
        set_camera(&camera);
        draw_grid(20, 1., WHITE, GRAY);

        let (_, mouse_wheel) = mouse_wheel(); // -1 backwards, +1 forward
        state.zoom += mouse_wheel * 0.02;

        function::draw(
            &state.expression,
            state.render_distance,
            state.distance_draw_cutoff,
            state.sample_size,
            state.root_splits,
            state.root_find_tries,
        );

        camera.position = state.position.into();

        // drop back to 2D so we can draw text/shapes on screen
        set_default_camera();

        if state.show_debug_info {
            ui::draw_debug_info(&state);
        }

        if is_key_down(KeyCode::D) {
            state.position.theta -= 3. * get_frame_time();
        }
        if is_key_down(KeyCode::A) {
            state.position.theta += 3. * get_frame_time();
        }

        if is_key_down(KeyCode::W) {
            state.position.phi -= 3. * get_frame_time();
        }
        if is_key_down(KeyCode::S) {
            state.position.phi += 3. * get_frame_time();
        }

        state.position.phi = state.position.phi.clamp(0.01, PI - 0.01);

        if is_key_pressed(KeyCode::F1) {
            state.show_debug_info = !state.show_debug_info;
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        ui::draw(&mut state);

        if is_key_pressed(KeyCode::Tab) {
            state.input_mode_enabled = !state.input_mode_enabled;
        }

        egui_macroquad::draw();
        next_frame().await;
    }
}
