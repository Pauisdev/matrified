use macroquad::prelude::*;

#[macroquad::main("Matrified")]
async fn main() {
    let mut camera = Camera3D {
        position: vec3(0., 20., -50.),
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
        let (_, mouse_wheel) = mouse_wheel();
        camera.position += mouse_wheel;
        set_default_camera();

        if show_debug_info {
            let debug_info = format!(
                "[camera] \n\
            * direction: ({}, {}, {}) \n\
            * position: ({}, {}, {})",
                camera.target.x,
                camera.target.y,
                camera.target.z,
                camera.position.x,
                camera.position.y,
                camera.position.z
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
