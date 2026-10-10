use macroquad::{
    prelude::*,
    ui::{Skin, hash, root_ui, widgets},
};
const FONT_SIZE: f32 = 32.;

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
    let mut expression = String::new();
    let skin = ui_skin();
    root_ui().push_skin(&skin);

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
            widgets::Window::new(hash!(), vec2(200., 100.), vec2(320., 400.))
                .label("Input menu")
                .titlebar(true)
                .ui(&mut *root_ui(), |ui| {
                    ui.input_text(hash!(), "Expression", &mut expression);
                });
        }

        if is_key_pressed(KeyCode::Tab) {
            input_mode_enabled = !input_mode_enabled;
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
    draw_multiline_text(debug_info, 0., 20., FONT_SIZE, None, WHITE);
}

fn traverse_points(a: &Vec3, b: &Vec3, delta: f32) -> Vec3 {
    *a + delta * (*b - *a)
}

fn ui_skin() -> Skin {
    let window_titlebar_style = root_ui()
        .style_builder()
        .font_size(FONT_SIZE as u16)
        .text_color(WHITE)
        .build();
    let label_style = root_ui()
        .style_builder()
        .font_size(FONT_SIZE as u16)
        .text_color(WHITE)
        .build();
    let editbox_style = root_ui()
        .style_builder()
        .font_size(FONT_SIZE as u16)
        .build();
    let window_style = root_ui()
        .style_builder()
        .font_size(FONT_SIZE as u16)
        .background(Image::gen_image_color(320, 400, DARKBLUE))
        .build();
    Skin {
        window_style,
        window_titlebar_style,
        label_style,
        editbox_style,
        title_height: FONT_SIZE,
        ..root_ui().default_skin()
    }
}
