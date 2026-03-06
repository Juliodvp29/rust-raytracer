use std::sync::Arc;
use rayon::prelude::*;
use rust_raytracer::{
    math::{Vec3, Color, Point3, random_f64, random_vec3},
    core::Ray,
    geometry::{Hittable, Sphere},
    scene::World,
    render::Camera,
    materials::{Lambertian, Metal, Dielectric},
    utils::to_rgb,
};

fn ray_color(ray: &Ray, world: &World, depth: u32) -> Color {
    if depth == 0 { return Color::zero(); }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.material.scatter(ray, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::zero();
    }

    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);
    Color::one() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn build_scene() -> World {
    let mut world = World::new();

    // Ground
    let mat_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground)));

    // Hundreds of random small spheres
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            // Skip spheres too close to the 3 feature spheres
            if (center - Point3::new(4.0, 0.2, 0.0)).length() <= 0.9 { continue; }

            let p = random_f64();
            let mat: Arc<dyn rust_raytracer::materials::Material> = if p < 0.8 {
                // 80% — Lambertian with random color
                let albedo = random_vec3(0.0, 1.0) * random_vec3(0.0, 1.0);
                Arc::new(Lambertian::new(albedo))
            } else if p < 0.95 {
                // 15% — Metal with random albedo and fuzz
                let albedo = random_vec3(0.5, 1.0);
                let fuzz   = random_f64() * 0.5;
                Arc::new(Metal::new(albedo, fuzz))
            } else {
                // 5% — Glass
                Arc::new(Dielectric::new(1.5))
            };

            world.add(Box::new(Sphere::new(center, 0.2, mat)));
        }
    }

    // Three feature spheres
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0), 1.0,
        Arc::new(Dielectric::new(1.5)),          // glass
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0), 1.0,
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))), // diffuse brown
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0), 1.0,
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)), // perfect mirror
    )));

    world
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32  = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 200;
    let max_depth: u32 = 50;

    let world = Arc::new(build_scene());

    // Classic final scene camera angle
    let look_from  = Point3::new(13.0, 2.0, 3.0);
    let look_at    = Point3::new( 0.0, 0.0, 0.0);
    let focus_dist = 10.0; // focus on the scene center
    let camera = Arc::new(Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        focus_dist,
    ));

    let total_pixels = (image_width * image_height) as usize;
    eprintln!("Rendering {}x{} @ {} spp on {} threads...",
        image_width, image_height, samples_per_pixel,
        rayon::current_num_threads());

    let pixels: Vec<(u8, u8, u8)> = (0..total_pixels)
        .into_par_iter()
        .map(|idx| {
            let i =  (idx as u32) % image_width;
            let j = image_height - 1 - (idx as u32) / image_width;

            let mut color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / (image_width  - 1) as f64;
                let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
                color += ray_color(&camera.get_ray(u, v), &world, max_depth);
            }
            to_rgb(color, samples_per_pixel)
        })
        .collect();

    rust_raytracer::utils::save_png(&pixels, image_width, image_height, "output.png");
    eprintln!("Done! Saved to output.png");
}