use macroquad::prelude::*;

use crate::math;

pub fn draw(expression: &String, render_distance: f32) {
    let f = parse(expression);
    if let Some(f) = f {
        render(&f, render_distance);
    };
}

fn parse(expression: &String) -> Option<impl Fn(f64) -> f64> {
    let Ok(expr) = expression.parse::<meval::Expr>() else {
        return None; // None if theres no expression or its malformed
    };

    let Ok(f) = expr.bind("x") else {
        return None; // None if expression contains variables other than `x`
    };

    Some(f)
}

fn render(f: &impl Fn(f64) -> f64, render_distance: f32) {
    let derivative_of_f = math::derivative(f);
    let root = find_root(
        derivative_of_f,
        -render_distance as f64,
        render_distance as f64,
    );
    if let Some(root) = root {
        println!("Root found! (x = {root})");
        let position = vec3(root as f32, 0., 0.);
        draw_cube(position, Vec3::ONE * 0.1, None, RED);
    } else {
        println!("No roots found")
    }

    //draw_text(text, x, y, font_size, color)
    /*for root in roots {
        let y = f(root);
        let position = vec3(root as f32, y as f32, 0.);
        draw_cube(position, Vec3::ONE * 0.1, None, RED);
    }*/
}

fn find_root(f: impl Fn(f64) -> f64, a: f64, b: f64) -> Option<f64> {
    let sign_of_a = f(a).signum();
    let sign_of_b = f(b).signum();

    let mut c = (a + b) / 2.;
    for i in 0..10 {
        let image_of_c = f(c);
        let sign_of_c = image_of_c.signum();
        println!("try: {i}, image: {image_of_c}");
        if is_roughly_zero(image_of_c) {
            return Some(image_of_c);
        }
        if sign_of_c != sign_of_a {
            c = (a + c) / 2.;
        } else if sign_of_c != sign_of_b {
            c = (c + b) / 2.;
        } else {
            // there's no roots
            return None;
        }
    }
    None
}

fn is_roughly_zero(value: f64) -> bool {
    value > -0.1 && value < 0.1
}
