use cached::cached;
use macroquad::prelude::*;

use crate::{math, state::State};

pub fn draw(state: &State) {
    let computed_points = compute_points(
        state.expression.clone(),
        state.render_distance,
        state.sample_size,
        state.root_splits,
        state.root_find_tries,
    );
    if computed_points.len() == 0 {
        return;
    }

    render(computed_points, state.distance_draw_cutoff);
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

fn render(points: Vec<(f32, f32)>, distance_draw_cutoff: f32) {
    let mut i = 0;
    while i < points.len() - 1 {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];

        // if difference between points is too high, assume they go to opposite infinities and don't connect them
        let dy = (y1 - y2).abs();
        if dy > distance_draw_cutoff {
            i += 1;
            continue;
        }

        let start = vec3(x1, y1 as f32, 0.);
        let end = vec3(x2, y2 as f32, 0.);
        draw_line_3d(start, end, YELLOW);
        i += 1;
    }
}

#[cached]
fn compute_points(
    expression: String,
    render_distance: i32,
    sample_size: i32,
    root_splits: i32,
    root_tries: i32,
) -> Vec<(f32, f32)> {
    let f = parse(&expression);
    let Some(f) = f else {
        return vec![];
    };
    let derivative_of_f = math::derivative(&f);
    // find roots on the derivative to get max and mins of the original function
    let mins_and_maxs = math::find_roots(
        &derivative_of_f,
        render_distance as f32,
        root_splits,
        root_tries,
    );
    let mut points_to_compute = vec![];
    points_to_compute.extend(mins_and_maxs);
    let gap = render_distance as f32 / sample_size as f32;
    for i in -sample_size..sample_size {
        let x = i as f32 * gap as f32;
        points_to_compute.push(x as f64);
    }
    points_to_compute.sort_by(|a, b| a.total_cmp(b));
    let mut computed_points = vec![];
    for point_to_compute in points_to_compute {
        computed_points.push((point_to_compute as f32, f(point_to_compute) as f32));
    }
    computed_points
}
