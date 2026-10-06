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
    let mut dots: Vec<Vec3> = vec![];

    loop {
        clear_background(BLACK);
        set_camera(&camera);

        draw_grid(20, 1., WHITE, GRAY);
        let start = vec3(7.0, 5.0, -10.0);
        let end = vec3(50.0, 10.0, 20.0);
        draw_line_3d(start, end, GREEN);
        let (_, mouse_wheel) = mouse_wheel(); // -1 backwards, +1 forward
        delta += mouse_wheel * 0.02;

        let A = initial_camera_position;
        let B = camera.target;
        let mut X = A + delta * (B - A);

        X.x = X.x * theta.cos() - X.z * theta.sin();
        X.z = X.x * theta.sin() + X.z * theta.cos();
        camera.position = X;

        let mut dot = camera.position;
        dot.y = 0.;

        dots.push(dot);
        for dot in dots.clone() {
            draw_cube(dot, Vec3::ONE, None, BLUE);
        }

        set_default_camera();

        if show_debug_info {
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
