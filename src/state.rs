use std::f32::consts::PI;

use crate::{app::EventLoop, function, math::SphericalCoordinates, scene, ui};
use macroquad::prelude::*;

const MOVEMENT_SPEED: f32 = 2.;
const ZOOM_SPEED: f32 = 0.3;
const MIN_ZOOM: f32 = 1.;

pub struct State {
    pub position: SphericalCoordinates,
    pub input_mode_enabled: bool,
    pub expression: String,
    pub render_distance: i32,
    pub root_splits: i32,
    pub root_find_tries: i32,
    pub sample_size: i32,
    pub distance_draw_cutoff: f32,
    pub camera: Camera3D,
}

impl State {
    pub fn new() -> State {
        State {
            position: SphericalCoordinates::from(10.),
            input_mode_enabled: false,
            expression: String::new(),
            render_distance: 100,
            root_splits: 100,
            root_find_tries: 10,
            sample_size: 1000,
            distance_draw_cutoff: 40.,
            camera: Camera3D {
                up: vec3(0., 1., 0.),
                ..Default::default()
            },
        }
    }

    fn handle_key_press(&mut self, event_loop: &mut EventLoop) {
        let delta = get_frame_time();

        let (_, mouse_wheel) = mouse_wheel(); // -1 backwards, +1 forward
        self.position.distance -= mouse_wheel * self.position.distance / ZOOM_SPEED * delta;

        if is_mouse_button_down(MouseButton::Right) {
            let mouse_delta = mouse_delta_position();
            self.position.theta -= MOVEMENT_SPEED * mouse_delta.x;
            self.position.phi += MOVEMENT_SPEED * mouse_delta.y;
        }

        self.position.phi = self.position.phi.clamp(0.01, PI - 0.01);
        self.position.distance = self.position.distance.clamp(MIN_ZOOM, f32::INFINITY);

        if is_key_pressed(KeyCode::Tab) {
            self.input_mode_enabled = !self.input_mode_enabled;
        }
        if is_key_pressed(KeyCode::Escape) {
            event_loop.exit();
        }
    }

    pub fn update(&mut self, event_loop: &mut EventLoop) {
        self.handle_key_press(event_loop);
        self.camera.position = self.position.into();
    }

    pub fn render(&self) {
        clear_background(BLACK);
        set_camera(&self.camera);
        scene::draw(&self);
        function::draw(&self);
    }

    pub fn ui(&mut self) {
        ui::draw(self);
        egui_macroquad::draw();
    }
}
