use crate::math::{Vec3, Point3};
use crate::core::Ray;

/// Stores all information about a single ray-object intersection.
/// This is filled in by `Hittable::hit()` and consumed by the shading code.
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    /// The exact 3D point where the ray intersected the surface.
    pub p: Point3,
    /// Surface normal at the hit point, always oriented *against* the incoming ray.
    /// This convention means shading code never has to flip the normal itself.
    pub normal: Vec3,
    /// The ray parameter `t` at the intersection: the point `p = ray.at(t)`.
    /// Smaller `t` means closer to the camera.
    pub t: f64,
    /// True if the ray hit the outer (front) face of the surface.
    /// False means the ray is inside the object (e.g., inside a glass sphere).
    pub front_face: bool,
}

impl HitRecord {
    /// Constructs a `HitRecord` and automatically determines face orientation.
    /// The `outward_normal` passed in always points away from the object center.
    /// If the ray is coming from outside (dot < 0), we keep the normal as-is;
    /// otherwise we flip it so it always opposes the ray direction.
    pub fn new(p: Point3, t: f64, ray: &Ray, outward_normal: Vec3) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        Self { p, normal, t, front_face }
    }
}

/// Trait that any 3D object must implement to participate in ray intersection tests.
/// `Send + Sync` make it safe to share objects across multiple rendering threads.
pub trait Hittable: Send + Sync {
    /// Tests whether the ray intersects this object within the interval [t_min, t_max].
    /// Returns `Some(HitRecord)` on a hit, or `None` if the ray misses.
    /// The t_min bound prevents self-intersections; t_max is tightened as closer
    /// hits are found to avoid rendering objects behind nearer ones.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
