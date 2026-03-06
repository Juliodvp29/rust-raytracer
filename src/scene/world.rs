use crate::core::Ray;
use crate::geometry::hittable::{Hittable, HitRecord};

/// The scene: an ordered list of all hittable objects.
///
/// Implements `Hittable` itself, so the renderer can test the entire scene
/// with a single `world.hit(ray, ...)` call instead of looping manually.
pub struct World {
    /// All objects in the scene. Using `Box<dyn Hittable>` allows mixing
    /// different object types (spheres, planes, meshes, etc.) in the same list.
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    /// Creates an empty world with no objects.
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    /// Adds a hittable object to the scene.
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for World {
    /// Finds the closest object hit by the ray within [t_min, t_max].
    ///
    /// The key trick: after each successful hit we shrink `closest` to `rec.t`,
    /// so subsequent objects are only accepted if they are even nearer.
    /// This guarantees we always return the front-most (visible) surface.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut result: Option<HitRecord> = None;

        for object in &self.objects {
            // Pass `closest` as t_max so we reject objects farther than the current best
            if let Some(rec) = object.hit(ray, t_min, closest) {
                closest = rec.t;   // tighten the upper bound
                result = Some(rec);
            }
        }

        result
    }
}
