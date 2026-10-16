mod function;
mod math;
mod state;
mod ui;
use crate::state::State;
use macroquad::prelude::*;
use std::f32::consts::PI;

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

        function::draw(
            &state.expression,
            state.render_distance,
            state.distance_draw_cutoff,
            state.sample_size,
            state.root_splits,
            state.root_find_tries,
        );

        camera.position = state.position.into();

        handle_movement(&mut state);

        if is_key_pressed(KeyCode::Tab) {
            state.input_mode_enabled = !state.input_mode_enabled;
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // drop back to 2D so we can draw text/shapes on screen
        set_default_camera();

        ui::draw(&mut state);
        egui_macroquad::draw();

        next_frame().await;
    }
}

const MOVEMENT_SPEED: f32 = 2.;
const ZOOM_SPEED: f32 = 30.;

fn handle_movement(state: &mut State) {
    let delta = get_frame_time();

    let (_, mouse_wheel) = mouse_wheel(); // -1 backwards, +1 forward
    state.position.distance -= mouse_wheel * ZOOM_SPEED * get_frame_time();

    if is_key_down(KeyCode::D) {
        state.position.theta -= MOVEMENT_SPEED * delta;
    }
    if is_key_down(KeyCode::A) {
        state.position.theta += MOVEMENT_SPEED * delta;
    }
    if is_key_down(KeyCode::W) {
        state.position.phi -= MOVEMENT_SPEED * get_frame_time();
    }
    if is_key_down(KeyCode::S) {
        state.position.phi += MOVEMENT_SPEED * get_frame_time();
    }

    state.position.phi = state.position.phi.clamp(0.01, PI - 0.01);
}
