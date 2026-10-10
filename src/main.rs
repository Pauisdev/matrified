use macroquad::prelude::*;

#[macroquad::main("Matrified")]
async fn main() {
    let initial_camera_position = vec3(0., 7., -20.);
    let mut theta: f32 = 0.;
    let mut delta = 0.;
    let mut camera = Camera3D {
        position: initial_camera_position,
        up: vec3(0., 1., 0.),
        ..Default::default()
    };
    set_fullscreen(true);
    let mut input_mode_enabled = false;
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

        if input_mode_enabled {
            open_input_menu();
        }

        if is_key_pressed(KeyCode::Tab) {
            input_mode_enabled = !input_mode_enabled;
        }

        next_frame().await;
    }
}

fn open_input_menu() {
    draw_input_menu_background();
}

fn draw_input_menu_background() {
    let horizontal_quadrant_size = screen_width() / 5.;
    let vertical_quadrant_size = screen_height() / 7.;

    let x1 = horizontal_quadrant_size;
    let x2 = horizontal_quadrant_size * 4.;
    let y1 = vertical_quadrant_size;
    let y2 = vertical_quadrant_size * 6.;

    draw_rectangle(x1, y1, x1 * 3., y1 * 5., Color::new(0., 0., 0., 0.9));
    draw_box(x1, y1, x2, y2);
}

fn draw_box(x1: f32, y1: f32, x2: f32, y2: f32) {
    let top_left_corner = vec2(x1, y1);
    let top_right_corner = vec2(x2, y1);
    let bottom_right_corner = vec2(x2, y2);
    let bottom_left_corner = vec2(x1, y2);

    connect_points(&top_left_corner, &top_right_corner);
    connect_points(&top_right_corner, &bottom_right_corner);
    connect_points(&bottom_right_corner, &bottom_left_corner);
    connect_points(&bottom_left_corner, &top_left_corner);
}

fn connect_points(a: &Vec2, b: &Vec2) {
    draw_line(a.x, a.y, b.x, b.y, 1., WHITE);
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

fn traverse_points(a: &Vec3, b: &Vec3, delta: f32) -> Vec3 {
    *a + delta * (*b - *a)
}
