use macroquad::prelude::*;

#[macroquad::main("Matrified")]
async fn main() {
    let initial_camera_position = vec3(0., 20., -50.);
    let mut theta: f32 = 0.;
    let mut delta = 0.;
    let mut camera = Camera3D {
        position: initial_camera_position,
        up: vec3(0., 1., 0.),
        ..Default::default()
    };
    set_fullscreen(true);
    let mut show_debug_info = true;

    loop {
        clear_background(BLACK);
        set_camera(&camera);

        draw_grid(20, 1., WHITE, GRAY);

        let (_, mouse_wheel) = mouse_wheel(); // -1 backwards, +1 forward
        delta += mouse_wheel * 0.02;

        let camera_position = traverse_points(&initial_camera_position, &camera.target, delta);
        let camera_position = rotate_x(&camera_position, theta);
        camera.position = camera_position;

        set_default_camera();

        if show_debug_info {
            draw_debug_info(&camera, delta, theta);
        }

        if is_key_down(KeyCode::D) {
            theta += 1. * get_frame_time();
        }
        if is_key_down(KeyCode::A) {
            theta -= 1. * get_frame_time();
        }

        if is_key_pressed(KeyCode::F1) {
            show_debug_info = !show_debug_info;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

fn rotate_x(original: &Vec3, angle: f32) -> Vec3 {
    vec3(
        original.x * angle.cos() - original.z * angle.sin(),
        original.y,
        original.x * angle.sin() + original.z * angle.cos(),
    )
}

fn draw_debug_info(camera: &Camera3D, delta: f32, theta: f32) {
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
    draw_multiline_text(debug_info, 0., 20., 32., None, WHITE);
}

fn traverse_points(A: &Vec3, B: &Vec3, delta: f32) -> Vec3 {
    *A + delta * (*B - *A)
}
