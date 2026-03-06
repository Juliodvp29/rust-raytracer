use crate::math::{Vec3, Point3};
use crate::core::Ray;

/// A simple pinhole camera that maps UV coordinates to rays in 3D space.
///
/// The camera uses a virtual viewport (a rectangle in front of it) to define
/// what portion of the scene is visible. Every pixel corresponds to one point
/// on that viewport, and a ray is traced from the camera origin through that point.
pub struct Camera {
    /// Position of the camera in world space (currently always at the origin).
    origin: Point3,
    /// The bottom-left corner of the viewport rectangle in world space.
    /// Computed once at construction so `get_ray` is very cheap.
    lower_left_corner: Point3,
    /// Full width vector of the viewport pointing to the right (X axis).
    horizontal: Vec3,
    /// Full height vector of the viewport pointing upward (Y axis).
    vertical: Vec3,
}

impl Camera {
    /// Creates a camera from high-level parameters.
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let viewport_width = aspect_ratio * viewport_height;

        let origin = Point3::zero();
        // The viewport spans from -horizontal/2 to +horizontal/2 horizontally
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        // Start at the origin, move left half the width, down half the height,
        // then forward (negative Z) by focal_length to reach the viewport's bottom-left corner
        let lower_left_corner = origin
            - horizontal * 0.5
            - vertical * 0.5
            - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    /// Generates a ray that passes through the viewport point at normalized coordinates (u, v).
    /// u ∈ [0, 1] is the horizontal position (0 = left, 1 = right).
    /// v ∈ [0, 1] is the vertical position (0 = bottom, 1 = top).
    /// The ray direction is not normalized here; normalization happens in `ray_color`
    /// when computing the sky gradient.
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner
            + self.horizontal * u
            + self.vertical * v
            - self.origin;
        Ray::new(self.origin, direction)
    }
}
