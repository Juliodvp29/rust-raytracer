use rayon::prelude::*;
use crate::math::{Color, random_f64};
use crate::core::Ray;
use crate::geometry::Hittable;
use crate::render::Camera;

pub struct Renderer {
    pub width:  u32,
    pub height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn render_sample(
        &self,
        camera: &Camera,
        world:  &dyn Hittable,
        max_depth: u32,
    ) -> Vec<Color> {
        let w = self.width;
        let h = self.height;

        (0..w * h)
            .into_par_iter()
            .map(|idx| {
                let i =  idx % w;
                let j = h - 1 - idx / w;
                let u = (i as f64 + random_f64()) / (w - 1) as f64;
                let v = (j as f64 + random_f64()) / (h - 1) as f64;
                ray_color(&camera.get_ray(u, v), world, max_depth)
            })
            .collect()
    }
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 { return Color::zero(); }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.material.scatter(ray, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::zero();
    }

    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);
    Color::one() * (1.0 - t) + Color::new(0.6, 0.8, 1.0) * t
}