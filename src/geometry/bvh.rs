use std::sync::Arc;
use crate::core::Ray;
use crate::math::Point3;
use super::hittable::{Hittable, HitRecord};


/// An Axis-Aligned Bounding Box used for BVH acceleration.
/// Different from geometry::Aabb — this one is NOT renderable, just a volume test.
#[derive(Clone, Copy)]
pub struct BoundingBox {
    pub min: Point3,
    pub max: Point3,
}

impl BoundingBox {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    /// Slab method: returns true if the ray intersects this box within [t_min, t_max]
    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
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
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min { return false; }
        }
        true
    }

    /// Returns a box that contains both self and other
    pub fn surrounding(a: BoundingBox, b: BoundingBox) -> BoundingBox {
        BoundingBox::new(
            Point3::new(
                a.min.x.min(b.min.x),
                a.min.y.min(b.min.y),
                a.min.z.min(b.min.z),
            ),
            Point3::new(
                a.max.x.max(b.max.x),
                a.max.y.max(b.max.y),
                a.max.z.max(b.max.z),
            ),
        )
    }
}

// ─── Trait extension: objects must provide their bounding box ─────────────────

pub trait Bounded: Hittable {
    fn bounding_box(&self) -> BoundingBox;
}


/// A node in the Bounding Volume Hierarchy tree.
///
/// Each node stores:
/// - Its own bounding box (wraps all children)
/// - Left and right children (either leaves or other BvhNodes)
///
/// Construction: sort objects by longest axis, split in half, recurse.
/// Traversal: if ray misses the node's box → skip entire subtree.
pub struct BvhNode {
    left:  Arc<dyn Bounded>,
    right: Arc<dyn Bounded>,
    bbox:  BoundingBox,
}

impl BvhNode {
    /// Build a BVH from a list of bounded objects.
    pub fn build(mut objects: Vec<Arc<dyn Bounded>>) -> Self {
        // Choose the axis with the largest span to split on
        let axis = longest_axis(&objects);

        match objects.len() {
            1 => {
                // Only one object: put it on both sides (leaf duplication)
                let obj = Arc::clone(&objects[0]);
                let bbox = obj.bounding_box();
                BvhNode { left: Arc::clone(&obj), right: obj, bbox }
            }
            2 => {
                // Two objects: one on each side
                let right = Arc::clone(&objects[1]);
                let left  = Arc::clone(&objects[0]);
                let bbox  = BoundingBox::surrounding(
                    left.bounding_box(), right.bounding_box()
                );
                BvhNode { left, right, bbox }
            }
            _ => {
                // Sort by chosen axis centroid, split in half, recurse
                objects.sort_by(|a, b| {
                    let ca = centroid(&a.bounding_box(), axis);
                    let cb = centroid(&b.bounding_box(), axis);
                    ca.partial_cmp(&cb).unwrap()
                });

                let mid = objects.len() / 2;
                let right_objs = objects.split_off(mid);
                let left_objs  = objects;

                let left:  Arc<dyn Bounded> = Arc::new(BvhNode::build(left_objs));
                let right: Arc<dyn Bounded> = Arc::new(BvhNode::build(right_objs));
                let bbox = BoundingBox::surrounding(
                    left.bounding_box(), right.bounding_box()
                );

                BvhNode { left, right, bbox }
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // If ray misses our bounding box → skip entire subtree (the key speedup)
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        // Test left child first
        let hit_left = self.left.hit(ray, t_min, t_max);

        // For right child, shrink t_max if left already hit something closer
        let t_max_right = hit_left.as_ref().map_or(t_max, |h| h.t);
        let hit_right = self.right.hit(ray, t_min, t_max_right);

        // Return whichever hit is closer (right wins if both hit, since t_max was shrunk)
        hit_right.or(hit_left)
    }
}

impl Bounded for BvhNode {
    fn bounding_box(&self) -> BoundingBox {
        self.bbox
    }
}

// ─── Bounded implementations for scene objects ───────────────────────────────

use crate::geometry::Sphere;
use crate::geometry::Aabb as RenderAabb;
use crate::math::Vec3;

impl Bounded for Sphere {
    fn bounding_box(&self) -> BoundingBox {
        let rv = Vec3::new(self.radius, self.radius, self.radius);
        BoundingBox::new(self.center - rv, self.center + rv)
    }
}

impl Bounded for RenderAabb {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(self.min, self.max)
    }
}


/// Returns the index of the axis (0=X, 1=Y, 2=Z) with the largest span across all objects
fn longest_axis(objects: &[Arc<dyn Bounded>]) -> usize {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];

    for obj in objects {
        let bb = obj.bounding_box();
        let vals = [
            (bb.min.x, bb.max.x),
            (bb.min.y, bb.max.y),
            (bb.min.z, bb.max.z),
        ];
        for (i, (lo, hi)) in vals.iter().enumerate() {
            min[i] = min[i].min(*lo);
            max[i] = max[i].max(*hi);
        }
    }

    let spans = [max[0]-min[0], max[1]-min[1], max[2]-min[2]];
    spans.iter()
         .enumerate()
         .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
         .map(|(i, _)| i)
         .unwrap_or(0)
}

/// Returns the centroid of a bounding box along a given axis
fn centroid(bb: &BoundingBox, axis: usize) -> f64 {
    match axis {
        0 => (bb.min.x + bb.max.x) * 0.5,
        1 => (bb.min.y + bb.max.y) * 0.5,
        _ => (bb.min.z + bb.max.z) * 0.5,
    }
}

impl Hittable for Arc<BvhNode> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.as_ref().hit(ray, t_min, t_max)
    }
}