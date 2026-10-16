use std::fmt::Display;

use macroquad::prelude::*;

pub fn rotate_x(original: &Vec3, yaw: f32, pitch: f32) -> Vec3 {
    let x = original.x;
    let z = original.y;
    let y = original.z;
    vec3(
        (yaw.cos() * pitch.cos()) * x + (-yaw.sin() * pitch.cos()) * y + pitch.sin() * z,
        yaw.sin() * x + yaw.cos() * y,
        (-pitch.sin() * yaw.cos()) * x + (-pitch.sin() * -yaw.sin()) * y + pitch.cos() * z,
    )
}

#[derive(Copy, Clone)]
pub struct SphericalCoordinates {
    pub distance: f32,
    pub phi: f32,
    pub theta: f32,
}

impl SphericalCoordinates {
    pub fn from_distance(distance: f32) -> Self {
        SphericalCoordinates {
            distance,
            phi: 1.,
            theta: 0.,
        }
    }
}

impl Into<Vec3> for SphericalCoordinates {
    fn into(self) -> Vec3 {
        vec3(
            self.distance * self.phi.sin() * self.theta.cos(),
            self.distance * self.phi.cos(),
            self.distance * self.phi.sin() * self.theta.sin(), // swap y with z because here Y points up
        )
    }
}

impl Display for SphericalCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:.2}, {:.2}°, {:.2}°)",
            self.distance, self.phi, self.theta
        )
    }
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

pub fn find_roots(
    f: &impl Fn(f64) -> f64,
    render_distance: f32,
    splits: i32,
    tries: i32,
) -> Vec<f64> {
    let gap = render_distance / splits as f32;
    let mut roots = vec![];
    for i in -splits..splits {
        let a = i as f32 * gap;
        let b = (i + 1) as f32 * gap;
        let root = find_root(&f, a as f64, b as f64, tries);
        if let Some(root) = root {
            roots.push(root);
        }
    }
    roots
}

/// Given a function `f` that meets the following criteria below, a range `[a,b]` and the amount of `tries` to do, finds a root in such range.
/// # Criteria
/// - `f` is continous in the range `[a,b]`
/// - `f` has a single root (or none)
/// # Caution
/// It doesn't actually find the true root, rather gets as close as `CLOSENESS_TO_ZERO` distance away from true zero.
///
/// If the range `[a,b]` is too big, it may not be able to find it in the amount of `tries` defined. In that case, consider either shortening the range or increasing the amount of tries.
fn find_root(f: &impl Fn(f64) -> f64, a: f64, b: f64, tries: i32) -> Option<f64> {
    let sign_of_a = f(a).signum();
    let sign_of_b = f(b).signum();

    let mut c = (a + b) / 2.;
    for _ in 0..tries {
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

const CLOSENESS_TO_ZERO: f64 = 0.1;

fn is_roughly_zero(value: f64) -> bool {
    value > -CLOSENESS_TO_ZERO && value < CLOSENESS_TO_ZERO
}
