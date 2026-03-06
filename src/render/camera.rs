use crate::math::{Vec3, Point3};
use crate::core::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let viewport_width = aspect_ratio * viewport_height;

        let origin = Point3::zero();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
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

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner
            + self.horizontal * u
            + self.vertical * v
            - self.origin;
        Ray::new(self.origin, direction)
    }
}
