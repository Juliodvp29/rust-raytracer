use crate::core::Ray;
use crate::geometry::hittable::{Hittable, HitRecord};
use crate::geometry::bvh::{BvhNode, Bounded};
use std::sync::Arc;

/// The scene: a list of hittable objects
pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    /// Builds a BVH from a separate list of bounded objects.
    /// Call this after building the scene to get O(log n) traversal.
    pub fn build_bvh(bounded: Vec<Arc<dyn Bounded>>) -> BvhNode {
        BvhNode::build(bounded)
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut result: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest) {
                closest = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}