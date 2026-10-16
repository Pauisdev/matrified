mod function;
mod math;
mod state;
mod ui;
use crate::state::State;
use macroquad::prelude::*;
use std::f32::consts::PI;
mod scene;

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

        scene::draw(&state);
        function::draw(&state);

        camera.position = state.position.into();

        handle_movement(&mut state);

        if is_key_pressed(KeyCode::Tab) {
            state.input_mode_enabled = !state.input_mode_enabled;
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        ui::draw(&mut state);
        egui_macroquad::draw();

        next_frame().await;
    }
}

const MOVEMENT_SPEED: f32 = 2.;
const ZOOM_SPEED: f32 = 0.3;
const MIN_ZOOM: f32 = 1.;

fn handle_movement(state: &mut State) {
    let delta = get_frame_time();

    let (_, mouse_wheel) = mouse_wheel(); // -1 backwards, +1 forward
    state.position.distance -= mouse_wheel * state.position.distance / ZOOM_SPEED * delta;

    if is_mouse_button_down(MouseButton::Right) {
        let mouse_delta = mouse_delta_position();
        state.position.theta -= MOVEMENT_SPEED * mouse_delta.x;
        state.position.phi += MOVEMENT_SPEED * mouse_delta.y;
    }

    state.position.phi = state.position.phi.clamp(0.01, PI - 0.01);
    state.position.distance = state.position.distance.clamp(MIN_ZOOM, f32::INFINITY);
}
