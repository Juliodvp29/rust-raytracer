use crate::math::{Vec3, Color};
use crate::core::Ray;
use crate::geometry::HitRecord;
use super::material::Material;
use crate::math::random_unit_vector;

pub struct Metal {
    /// Color of the metal surface
    pub albedo: Color,
    /// Fuzz = 0.0 → perfect mirror. Fuzz = 1.0 → very blurry reflection.
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        // Reflect the incoming ray around the surface normal
        let reflected = ray_in.direction.normalize().reflect(hit.normal);

        // Add fuzz: slightly randomize the reflected direction
        let scattered = Ray::new(
            hit.p,
            reflected + random_unit_vector() * self.fuzz,
        );

        // Only scatter if the reflected ray goes away from the surface
        if scattered.direction.dot(hit.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None // Ray went into the surface — absorbed
        }
    }
}