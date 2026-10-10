mod math;
mod ui;
use macroquad::prelude::*;
use ui::State;

const INITIAL_CAMERA_POSITION: Vec3 = vec3(0., 7., 20.);

#[macroquad::main("Matrified")]
async fn main() {
    let mut theta: f32 = 0.;
    let mut delta = 0.;
    let mut camera = Camera3D {
        position: INITIAL_CAMERA_POSITION,
        up: vec3(0., 1., 0.),
        ..Default::default()
    };

    set_fullscreen(true);
    let mut input_mode_enabled = false;
    let mut show_debug_info = true;

    let mut state = State::new();
    ui::push_styles();

    loop {
        clear_background(BLACK);

        set_camera(&camera);

        draw_grid(20, 1., WHITE, GRAY);

        let (_, mouse_wheel) = mouse_wheel(); // -1 backwards, +1 forward
        delta += mouse_wheel * 0.02;

        let camera_position =
            math::traverse_points(&INITIAL_CAMERA_POSITION, &camera.target, delta);
        let camera_position = math::rotate_x(&camera_position, theta);
        camera.position = camera_position;

        render_function(&state.expression);

        // drop back to 2D so we can draw text/shapes on screen
        set_default_camera();

        if show_debug_info {
            ui::draw_debug_info(&camera, delta, theta);
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
            ui::draw(&mut state);
        }

        if is_key_pressed(KeyCode::Tab) {
            input_mode_enabled = !input_mode_enabled;
        }

        next_frame().await;
    }
}

fn render_function(expression: &String) {
    let mut x = -100.;
    while x < 100. {
        let Ok(expr) = expression.parse::<meval::Expr>() else {
            return; // exit out if theres no expression or its malformed
        };
        let Ok(f) = expr.bind("x") else {
            return; // exit out if expression contains variables other than `x`
        };
        let y = f(x);
        let position = vec3(x as f32, y as f32, 0.);
        draw_cube(position, Vec3::ONE * 0.1, None, RED);
        x += 0.02;
    }
}
