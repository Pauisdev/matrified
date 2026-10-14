use macroquad::prelude::*;

pub fn rotate_x(original: &Vec3, angle: f32) -> Vec3 {
    vec3(
        original.x * angle.cos() - original.z * angle.sin(),
        original.y,
        original.x * angle.sin() + original.z * angle.cos(),
    )
}

///
/// Traverses the line intersecting both points.
/// # Usage
/// `λ = 0` will return A whilst `λ = 1` returns B.
/// # Caution
/// If both points are the same A + λ(B-A) turns into A + λ(A-A) = A so it will always return the original point.

pub fn traverse_points(a: &Vec3, b: &Vec3, lambda: f32) -> Vec3 {
    *a + lambda * (*b - *a)
}

const H: f64 = 1e-5;

pub fn derivative(f: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
    move |x| (f(x + H) - f(x)) / H
}

pub fn find_roots(f: &impl Fn(f64) -> f64, render_distance: f32) -> Vec<f64> {
    let splits = 100;
    let gap = render_distance / splits as f32;
    let mut roots = vec![];
    for i in -splits..splits {
        let a = i as f32 * gap;
        let b = (i + 1) as f32 * gap;
        let root = find_root(&f, a as f64, b as f64);
        if let Some(root) = root {
            roots.push(root);
        }
    }
    roots
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
