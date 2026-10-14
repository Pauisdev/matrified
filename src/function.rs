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

    let mut points_to_draw = vec![];
    points_to_draw.extend(mins_and_maxs);
    let splits = 1000;
    let gap = render_distance / splits as f32;
    for i in -splits..splits {
        let x = i as f32 * gap;
        points_to_draw.push(x as f64);
    }
    points_to_draw.sort_by(|a, b| a.total_cmp(b));

    let mut i = 0;
    while i < points_to_draw.len() - 1 {
        let current = points_to_draw[i];
        let next = points_to_draw[i + 1];
        let y1 = f(current);
        let y2 = f(next);

        // if difference between points is too high, assume they go to opposite infinities and don't connect them
        let dy = (y1 - y2).abs();
        if dy > 40. {
            i += 1;
            continue;
        }

        let start = vec3(current as f32, y1 as f32, 0.);
        let end = vec3(next as f32, y2 as f32, 0.);
        draw_line_3d(start, end, YELLOW);
        i += 1;
    }
}
