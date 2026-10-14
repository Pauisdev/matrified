use macroquad::prelude::*;

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
    let mut x = -render_distance;
    while x < render_distance {
        let y = f(x as f64);
        let position = vec3(x as f32, y as f32, 0.);
        draw_cube(position, Vec3::ONE * 0.1, None, RED);

        x += 0.02;
    }
}
