use crate::math::Color;
use crate::core::Ray;
use crate::geometry::HitRecord;
use super::material::Material;
use crate::math::random_unit_vector;

pub struct Lambertian {
    /// The albedo is how much light the surface reflects per channel (R, G, B).
    /// A white surface (1,1,1) reflects everything. Black (0,0,0) absorbs all.
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        // Scatter in a random direction near the surface normal (Lambertian distribution)
        let mut scatter_direction = hit.normal + random_unit_vector();

        // Catch degenerate case: if random vector exactly cancels the normal
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}