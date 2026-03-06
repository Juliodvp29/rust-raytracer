use crate::core::Ray;
use crate::geometry::Hittable;
use crate::math::{random_f64, Color};
use crate::render::Camera;
use rayon::prelude::*;

pub struct Renderer {
    pub width: u32,
    pub height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Renders one full-resolution sample (1 ray per pixel, jittered).
    pub fn render_sample(
        &self,
        camera: &Camera,
        world: &dyn Hittable,
        max_depth: u32,
    ) -> Vec<Color> {
        let w = self.width;
        let h = self.height;

        (0..w * h)
            .into_par_iter()
            .map(|idx| {
                let i = idx % w;
                let j = h - 1 - idx / w;
                let u = (i as f64 + random_f64()) / (w - 1) as f64;
                let v = (j as f64 + random_f64()) / (h - 1) as f64;
                ray_color(&camera.get_ray(u, v), world, max_depth)
            })
            .collect()
    }

    /// Renders at `1/scale` resolution and upscales back to full size via
    /// nearest-neighbor. Useful as a fast preview during camera movement:
    /// the image is blurry but has far less Monte-Carlo noise than a
    /// full-resolution 1-spp render.
    ///
    /// `scale` must be >= 1. Common values: 2 (half-res) or 4 (quarter-res).
    pub fn render_sample_scaled(
        &self,
        camera: &Camera,
        world: &dyn Hittable,
        max_depth: u32,
        scale: u32,
    ) -> Vec<Color> {
        let full_w = self.width;
        let full_h = self.height;

        let scale = scale.max(1);
        let low_w = (full_w / scale).max(1);
        let low_h = (full_h / scale).max(1);

        // Render at low resolution
        let low_buf: Vec<Color> = (0..low_w * low_h)
            .into_par_iter()
            .map(|idx| {
                let i = idx % low_w;
                let j = low_h - 1 - idx / low_w;
                let u = (i as f64 + random_f64()) / (low_w - 1).max(1) as f64;
                let v = (j as f64 + random_f64()) / (low_h - 1).max(1) as f64;
                ray_color(&camera.get_ray(u, v), world, max_depth)
            })
            .collect();

        // Nearest-neighbour upscale to full resolution
        (0..full_w * full_h)
            .into_par_iter()
            .map(|idx| {
                let px = idx % full_w;
                let py = idx / full_w;
                let lx = (px * low_w / full_w).min(low_w - 1);
                let ly = (py * low_h / full_h).min(low_h - 1);
                low_buf[(ly * low_w + lx) as usize]
            })
            .collect()
    }
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::zero();
    }

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
