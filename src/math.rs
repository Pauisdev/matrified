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
