use crate::math::{Color, random_f64};
use crate::core::Ray;
use crate::geometry::HitRecord;
use super::material::Material;

pub struct Dielectric {
    /// Index of refraction (η). Common values:
    /// Air = 1.0, Glass = 1.5, Diamond = 2.4, Water = 1.33
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    /// Schlick approximation for reflectance.
    /// Returns the probability that the ray reflects instead of refracts.
    /// At steep angles (grazing incidence) glass looks like a mirror — this models that.
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        // Glass doesn't absorb light — attenuation is always white
        let attenuation = Color::one();

        // If the ray hits the outside face, it goes from air (1.0) into glass (ir).
        // If it hits the inside face, it goes from glass back into air.
        let refraction_ratio = if hit.front_face { 1.0 / self.ir } else { self.ir };

        let unit_dir = ray_in.direction.normalize();
        let cos_theta = (-unit_dir).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Total internal reflection: if sin_theta * ratio > 1, can't refract → must reflect
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        // Also reflect probabilistically based on Schlick at shallow angles
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > random_f64()
        {
            unit_dir.reflect(hit.normal)
        } else {
            unit_dir.refract(hit.normal, refraction_ratio)
        };

        Some((Ray::new(hit.p, direction), attenuation))
    }
}