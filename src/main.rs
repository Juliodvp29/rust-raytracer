use std::sync::Arc;
use rust_raytracer::{
    math::{Color, Point3, random_f64},
    core::Ray,
    geometry::{Hittable, Sphere},
    scene::World,
    render::Camera,
    materials::{Lambertian, Metal},
    utils::to_rgb,
};

// ─── Ray color (recursive) ───────────────────────────────────────────────────

fn ray_color(ray: &Ray, world: &World, depth: u32) -> Color {
    // Exceeded ray bounce limit — no more light gathered
    if depth == 0 {
        return Color::zero();
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        // Ask the material what to do with this ray
        if let Some((scattered, attenuation)) = rec.material.scatter(ray, &rec) {
            // Multiply attenuation color by the color of the scattered ray
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::zero(); // absorbed
    }

    // Background gradient
    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);
    Color::one() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}


fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100; // antialiasing
    let max_depth: u32 = 50;          // max ray bounces

    // Materials
    let mat_ground   = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center   = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_left     = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let mat_right    = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // Scene
    let mut world = World::new();
    world.add(Box::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Box::new(Sphere::new(Point3::new( 0.0,    0.0, -1.0),   0.5, mat_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, mat_left)));
    world.add(Box::new(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, mat_right)));

    // Camera
    let camera = Camera::new(aspect_ratio, 2.0, 1.0);

    // Render
    let mut pixels = Vec::with_capacity((image_width * image_height) as usize);
    eprintln!("Rendering {}x{} @ {} samples/px...", image_width, image_height, samples_per_pixel);


    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut color = Color::zero();

            // Antialiasing: accumulate N samples with random subpixel offsets
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
                let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world, max_depth);
            }

            pixels.push(to_rgb(color, samples_per_pixel));
        }
    }

    // Save PNG
    rust_raytracer::utils::save_png(&pixels, image_width, image_height, "output.png");

    // Still output PPM to stdout for backward compatibility if redirected
    println!("P3\n{} {}\n255", image_width, image_height);
    for (r, g, b) in &pixels {
        println!("{} {} {}", r, g, b);
    }

    eprintln!("Done! Image saved to output.png");
}