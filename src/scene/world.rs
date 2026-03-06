use crate::core::Ray;
use crate::geometry::hittable::{Hittable, HitRecord};

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
