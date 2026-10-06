use macroquad::prelude::*;

#[macroquad::main("Matrified")]
async fn main() {
    let initial_camera_position = vec3(0., 20., -50.);
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
        let start = vec3(7.0, 5.0, -10.0);
        let end = vec3(50.0, 10.0, 20.0);
        draw_line_3d(start, end, GREEN);
        let (_, mouse_wheel) = mouse_wheel(); // -1 backwards, +1 forward
        delta += mouse_wheel * 0.02;

        let A = initial_camera_position;
        let B = camera.target;
        let X = A + delta * (B - A);

        camera.position = X;

        set_default_camera();

        if show_debug_info {
            let debug_info = format!(
                "[camera] \n\
            * direction: ({}, {}, {}) \n\
            * position: ({}, {}, {}) \n\n\
            [general] \n\
            * fps: {} \n\
            * delta: {}",
                camera.target.x,
                camera.target.y,
                camera.target.z,
                camera.position.x,
                camera.position.y,
                camera.position.z,
                get_fps(),
                delta
            );
            draw_multiline_text(debug_info, 0., 20., 32., None, WHITE);
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
