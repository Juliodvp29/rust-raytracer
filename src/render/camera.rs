use crate::math::{Vec3, Point3, random_in_unit_disk};
use crate::core::Ray;

/// Camera represents the virtual eye in the scene, handling projection and depth of field.
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect * viewport_height;

        // Calculate the camera's local coordinate system (w, u, v)
        // w is the opposite of the look direction
        let w = (look_from - look_at).normalize();
        // u is the "right" vector
        let u = vup.cross(w).normalize();
        // v is the "up" vector relative to the camera
        let v = w.cross(u);

        let origin = look_from;
        // The horizontal and vertical vectors of the viewport, scaled by focus distance
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        
        // Calculate the world-space position of the bottom-left corner of the viewport
        let lower_left_corner = origin
            - horizontal * 0.5
            - vertical * 0.5
            - w * focus_dist;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    /// Generates a Ray for the given screen coordinates (s, t).
    /// s and t range from 0.0 to 1.0.
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        // Depth-of-field: pick a random point in the lens disk
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        
        // Ray starts from the origin (plus lens offset) and points towards the pixel on the focus plane
        let direction = self.lower_left_corner
            + self.horizontal * s
            + self.vertical * t
            - self.origin - offset;
            
        Ray::new(self.origin + offset, direction)
    }
}