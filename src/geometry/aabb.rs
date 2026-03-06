use crate::math::{Vec3, Point3};
use crate::core::Ray;
use crate::materials::Material;
use super::hittable::{Hittable, HitRecord};
use std::sync::Arc;

/// An Axis-Aligned Bounding Box used directly as a renderable object (a box/cube).
///
/// Defined by two corners: `min` (bottom-left-back) and `max` (top-right-front).
/// Intersection uses the slab method: for each axis, compute the ray's entry/exit t,
/// then take the overlap of all three intervals.
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
    pub material: Arc<dyn Material>,
}

impl Aabb {
    /// Create a box from two opposite corners.
    pub fn new(min: Point3, max: Point3, material: Arc<dyn Material>) -> Self {
        Self { min, max, material }
    }

    /// Helper: create a unit cube at position (x, y, z) with a given size.
    pub fn unit(origin: Point3, size: f64, material: Arc<dyn Material>) -> Self {
        Self::new(origin, origin + Vec3::new(size, size, size), material)
    }
}

impl Hittable for Aabb {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t_enter = t_min;
        let mut t_exit  = t_max;

        // ── Slab method: test each axis ───────────────────────────────────
        let axes = [
            (ray.origin.x, ray.direction.x, self.min.x, self.max.x),
            (ray.origin.y, ray.direction.y, self.min.y, self.max.y),
            (ray.origin.z, ray.direction.z, self.min.z, self.max.z),
        ];

        for (orig, dir, box_min, box_max) in axes {
            let inv = 1.0 / dir;
            let mut t0 = (box_min - orig) * inv;
            let mut t1 = (box_max - orig) * inv;
            if inv < 0.0 { std::mem::swap(&mut t0, &mut t1); }
            t_enter = t_enter.max(t0);
            t_exit  = t_exit.min(t1);
            if t_exit <= t_enter { return None; }
        }

        // ── Determine which face was hit and compute the normal ───────────
        let t = t_enter;
        let p = ray.at(t);

        // Find the closest face by comparing distances from hit point to each face
        let eps = 1e-6;
        let outward_normal = if (p.x - self.min.x).abs() < eps {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (p.x - self.max.x).abs() < eps {
            Vec3::new( 1.0, 0.0, 0.0)
        } else if (p.y - self.min.y).abs() < eps {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (p.y - self.max.y).abs() < eps {
            Vec3::new(0.0,  1.0, 0.0)
        } else if (p.z - self.min.z).abs() < eps {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0,  1.0)
        };

        Some(HitRecord::new(p, t, ray, outward_normal, Arc::clone(&self.material)))
    }
}