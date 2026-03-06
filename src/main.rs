use std::sync::Arc;
use rust_raytracer::{
    math::{Vec3, Color, Point3, random_f64},
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

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;
    let max_depth: u32 = 50;

    // Materials
    let mat_ground  = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center  = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left    = Arc::new(Dielectric::new(1.5));           // glass
    let mat_bubble  = Arc::new(Dielectric::new(1.0 / 1.5));    // hollow inside (negative radius trick)
    let mat_right   = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    // Scene
    let mut world = World::new();
    world.add(Box::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, mat_ground)));
    world.add(Box::new(Sphere::new(Point3::new( 0.0,    0.0, -1.0),   0.5, mat_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, mat_left)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),  -0.4, mat_bubble))); // hollow glass
    world.add(Box::new(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, mat_right)));

    // Camera — posicionada con look_from/look_at + ligero depth of field
    let look_from = Point3::new(3.0, 3.0, 2.0);
    let look_at   = Point3::new(0.0, 0.0, -1.0);
    let focus_dist = (look_from - look_at).length();

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0), // world up
        20.0,                      // vfov
        aspect_ratio,
        0.1,                       // aperture (pequeño para blur sutil)
        focus_dist,
    );

    // Render
    let mut pixels = Vec::with_capacity((image_width * image_height) as usize);
    eprintln!("Rendering {}x{} @ {} spp...", image_width, image_height, samples_per_pixel);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
                let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
                color += ray_color(&camera.get_ray(u, v), &world, max_depth);
            }
            pixels.push(to_rgb(color, samples_per_pixel));
        }
    }

    rust_raytracer::utils::save_png(&pixels, image_width, image_height, "output.png");
    eprintln!("Done! Saved to output.png");
}