use crate::math::{Vec3, Point3};

/// A ray in 3D space, defined by an origin point and a direction vector.
///
/// In ray tracing every pixel is shaded by casting at least one ray from the camera
/// through the scene. This struct carries the origin and direction needed to
/// evaluate any point along that ray.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    /// Starting point of the ray in world space (usually the camera position).
    pub origin: Point3,
    /// Direction the ray is travelling. Not required to be a unit vector,
    /// but many routines normalize it when computing the sky gradient.
    pub direction: Vec3,
}

impl Ray {
    /// Creates a new ray with the given origin and direction.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Evaluates the ray at parameter `t`: returns the point P(t) = origin + t * direction.
    /// - t = 0 → the origin itself
    /// - t > 0 → a point ahead of the origin (in the direction of travel)
    /// - t < 0 → behind the origin (used to filter out backface hits)
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
