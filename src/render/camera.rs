use crate::math::{Vec3, Point3, random_in_unit_disk};
use crate::core::Ray;

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

        // Calculate the camera's orthonormal basis (w, u, v)
        // w points away from the target
        let w = (look_from - look_at).normalize();
        // u is the "right" vector perpendicular to w and vup
        let u = vup.cross(w).normalize();
        // v is the relative "up" vector completing the basis
        let v = w.cross(u);

        let origin = look_from;
        // The viewportvectors are scaled by focus_dist so that they lie on the plane of focus
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        
        // Final bottom-left corner calculation in world space
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

    /// Generates a Ray for the given screen coordinates (s, t) in [0, 1].
    /// Includes defocus blur by sampling a random point on the virtual lens.
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        // Defocus blur: pick a random point in the lens disk
        let rd = random_in_unit_disk() * self.lens_radius;
        // Offset the ray origin so it starts from a random part of the lens
        let offset = self.u * rd.x + self.v * rd.y;
        
        // The ray points from the offset origin towards the specific pixel on the focus plane
        let direction = self.lower_left_corner
            + self.horizontal * s
            + self.vertical * t
            - self.origin - offset;
            
        Ray::new(self.origin + offset, direction)
    }
}