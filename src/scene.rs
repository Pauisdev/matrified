use macroquad::prelude::*;

pub fn draw() {
    draw_grid(20, 1., WHITE, GRAY);
    draw_arrows();
}

fn draw_arrows() {
    let size = 11.;
    let thickness = 0.05;
    draw_arrow(Vec3::X, size, thickness, RED);
    draw_arrow(Vec3::Y, size, thickness, GREEN);
    draw_arrow(Vec3::Z, size, thickness, BLUE);
}

fn draw_arrow(direction: Vec3, size: f32, thickness: f32, color: Color) {
    let thickness_vec = Vec3::ONE * thickness;
    draw_cube(
        direction * Vec3::ONE * (size / 2.),
        vec3(size, size, size) * direction + thickness_vec - thickness_vec * direction,
        None,
        color,
    );
}
