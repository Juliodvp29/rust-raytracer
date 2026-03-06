use crate::math::{Vec3, Color};
use crate::core::Ray;
use crate::geometry::HitRecord;

/// Any material must implement this trait.
/// scatter decides if the ray continues and in which direction.
pub trait Material: Send + Sync {
    /// Returns Some((scattered_ray, attenuation)) if the ray scatters,
    /// or None if the ray is fully absorbed.
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}