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
    // find roots on the derivative to get max and mins of the original function
    let mins_and_maxs = math::find_roots(&derivative_of_f, render_distance);

    // render with previous method for debugging purposes
    let mut x = -render_distance;
    while x < render_distance {
        let y = f(x as f64);
        let position = vec3(x, y as f32, -3.);
        draw_cube(position, Vec3::ONE * 0.1, None, RED);
        x += 0.02;
    }

    // render with new method

    //if function has no min/max points
    if mins_and_maxs.len() == 0 {
        return;
    }
    let mut i = 0;
    while i < mins_and_maxs.len() - 1 {
        let current = mins_and_maxs[i];
        let next = mins_and_maxs[i + 1];
        let middle = (current + next) / 2.;
        draw_line_following_function(f, current, middle);
        draw_line_following_function(f, middle, next);

        i += 1;
    }
}

/// Given two x values and a function, applies them to the function and draws a line between.
fn draw_line_following_function(f: impl Fn(f64) -> f64, x1: f64, x2: f64) {
    let start = vec3(x1 as f32, f(x1) as f32, 0.);
    let end = vec3(x2 as f32, f(x2) as f32, 0.);
    draw_line_3d(start, end, YELLOW);
}
