use crate::math::Point3;
use crate::core::Ray;
use super::hittable::{Hittable, HitRecord};

/// A sphere defined by its center point and radius.
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    /// Tests if the ray intersects this sphere using the analytic solution of a quadratic equation.
    ///
    /// A ray P(t) = origin + t*direction lies on the sphere's surface when:
    ///   |P(t) - center|² = radius²
    /// Expanding this gives a quadratic in t:  a·t² + 2b·t + c = 0
    ///   a = |direction|²
    ///   b = (origin - center) · direction   (stored as half_b to avoid 2× factors)
    ///   c = |origin - center|² - radius²
    /// discriminant = b² - a·c:
    ///   < 0 → no real solution (ray misses the sphere)
    ///   ≥ 0 → two solutions (entry and exit). We pick the nearest one inside [t_min, t_max].
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Vector from sphere center to ray origin
        let oc = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        // Negative discriminant means the ray completely misses the sphere
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Try the closer root (entry point) first; if it's out of range try the farther one (exit)
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = ray.at(root);
        // Outward normal: unit vector pointing from the sphere center to the hit point
        let outward_normal = (p - self.center) * (1.0 / self.radius);

        Some(HitRecord::new(p, root, ray, outward_normal))
    }
}
