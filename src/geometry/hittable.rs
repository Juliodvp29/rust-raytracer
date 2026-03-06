use crate::math::{Vec3, Point3};
use crate::core::Ray;

/// All information about a ray-object intersection
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    /// Point of intersection in world space
    pub p: Point3,
    /// Surface normal at the hit point (always points against the ray)
    pub normal: Vec3,
    /// Ray parameter t at the intersection
    pub t: f64,
    /// True if ray hits the outside face of the surface
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, ray: &Ray, outward_normal: Vec3) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        Self { p, normal, t, front_face }
    }
}

/// Trait for any object that can be intersected by a ray
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
