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

    let splits = 100;
    let gap = render_distance / splits as f32;
    let mut roots = vec![];
    for i in -splits..splits {
        let a = i as f32 * gap;
        let b = (i + 1) as f32 * gap;
        let root = find_root(&derivative_of_f, a as f64, b as f64);
        if let Some(root) = root {
            roots.push(root);
        }
    }

    let mut x = -render_distance;
    while x < render_distance {
        let y = f(x as f64);
        let position = vec3(x, y as f32, 0.);
        draw_cube(position, Vec3::ONE * 0.1, None, RED);
        x += 0.02;
    }

    for root in roots.clone() {
        let y = f(root);
        let position = vec3(root as f32, y as f32, 0.);
        draw_cube(position, Vec3::ONE * 0.5, None, YELLOW);
    }
}

/// Given a function `f` that meets the following criteria below and a range `[a,b]`, finds a root in such range.
/// # Criteria
/// - `f` is continous in the range `[a,b]`
/// - `f` has a single root (or none)
/// # Caution
/// It doesn't actually find the true root, rather gets as close up to `K` distance away from true zero.
///
/// If the range `[a,b]` is too big, it may not be able to find it in `0..10` tries. In that case, consider shortening the range.
fn find_root(f: &impl Fn(f64) -> f64, a: f64, b: f64) -> Option<f64> {
    let sign_of_a = f(a).signum();
    let sign_of_b = f(b).signum();

    let mut c = (a + b) / 2.;
    for _ in 0..10 {
        let image_of_c = f(c);
        let sign_of_c = image_of_c.signum();
        if is_roughly_zero(image_of_c) {
            return Some(c);
        }
        if sign_of_c != sign_of_a {
            c = (a + c) / 2.; // the root must be in the range [a,c]
        } else if sign_of_c != sign_of_b {
            c = (c + b) / 2.; // the root must be in the range [c,b]
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
