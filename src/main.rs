use std::sync::Arc;
use rayon::prelude::*;
use rust_raytracer::{
    math::{Vec3, Color, Point3, random_f64},
    core::Ray,
    geometry::{Hittable, Aabb},
    scene::World,
    render::Camera,
    materials::{Lambertian, Metal, Dielectric, Material},
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

    // Brighter sky gradient — more light in the scene
    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0);
    Color::one() * (1.0 - t) + Color::new(0.6, 0.8, 1.0) * t
}

fn cube(world: &mut World, x: f64, y: f64, z: f64, mat: Arc<dyn Material>) {
    let s = 0.88;
    world.add(Box::new(Aabb::new(
        Point3::new(x,     y,     z    ),
        Point3::new(x + s, y + s, z + s),
        mat,
    )));
}

fn piece(
    world: &mut World,
    base_x: f64, base_y: f64, base_z: f64,
    blocks: &[(i32, i32)],
    mat: Arc<dyn Material>,
) {
    for (dx, dy) in blocks {
        cube(world, base_x + *dx as f64, base_y + *dy as f64, base_z,
             Arc::clone(&mat));
    }
}

fn build_scene() -> World {
    let mut world = World::new();

    // ── Floor: light gray diffuse → bounces lots of indirect light ───────
    let mat_floor = Arc::new(Lambertian::new(Color::new(0.82, 0.82, 0.82)));
    world.add(Box::new(Aabb::new(
        Point3::new(-7.0, -0.15, -4.0),
        Point3::new( 7.0,  0.0,   6.0),
        mat_floor,
    )));

    // ── Back wall: light warm white ───────────────────────────────────────
    let mat_wall = Arc::new(Lambertian::new(Color::new(0.88, 0.86, 0.82)));
    world.add(Box::new(Aabb::new(
        Point3::new(-7.0, 0.0, -4.1),
        Point3::new( 7.0, 14.0, -4.0),
        mat_wall,
    )));

    // ── Left wall: soft blue-white ────────────────────────────────────────
    let mat_left: Arc<dyn Material> =
        Arc::new(Lambertian::new(Color::new(0.78, 0.82, 0.90)));
    world.add(Box::new(Aabb::new(
        Point3::new(-5.1, 0.0, -4.0),
        Point3::new(-5.0, 14.0, 6.0),
        Arc::clone(&mat_left),
    )));

    // ── Right wall: soft peach-white ──────────────────────────────────────
    let mat_right: Arc<dyn Material> =
        Arc::new(Lambertian::new(Color::new(0.90, 0.82, 0.78)));
    world.add(Box::new(Aabb::new(
        Point3::new(5.0, 0.0, -4.0),
        Point3::new(5.1, 14.0, 6.0),
        Arc::clone(&mat_right),
    )));

    // ════════════════════════════════════════════════════════════════════
    // TETRIS PIECES
    // Heavy use of glass and mirror to show off ray tracing
    // ════════════════════════════════════════════════════════════════════

    // ── Row 0: I-piece — perfect mirror, horizontal ───────────────────────
    // Shows reflections of everything above it
    let mat_i = Arc::new(Metal::new(Color::new(0.95, 0.95, 0.98), 0.0));
    piece(&mut world, -2.0, 0.0, 0.0,
          &[(0,0),(1,0),(2,0),(3,0)], mat_i);

    // ── Row 0-1: O-piece — gold mirror ───────────────────────────────────
    let mat_o = Arc::new(Metal::new(Color::new(0.83, 0.68, 0.22), 0.04));
    piece(&mut world, 2.0, 0.0, 0.0,
          &[(0,0),(1,0),(0,1),(1,1)], mat_o);

    // ── Row 1-2: T-piece — clear glass ───────────────────────────────────
    // Refracts the colorful pieces behind/around it
    let mat_t = Arc::new(Dielectric::new(1.5));
    piece(&mut world, -4.0, 1.0, 0.0,
          &[(0,0),(1,0),(2,0),(1,1)], mat_t);

    // ── Row 2-3: S-piece — cyan diffuse ──────────────────────────────────
    // One diffuse piece for color contrast
    let mat_s = Arc::new(Lambertian::new(Color::new(0.05, 0.75, 0.85)));
    piece(&mut world, -1.0, 2.0, 0.0,
          &[(0,0),(1,0),(1,1),(2,1)], mat_s);

    // ── Row 2-4: L-piece — brushed steel ─────────────────────────────────
    let mat_l = Arc::new(Metal::new(Color::new(0.70, 0.72, 0.75), 0.12));
    piece(&mut world, 2.0, 2.0, 0.0,
          &[(0,0),(0,1),(0,2),(1,0)], mat_l);

    // ── Row 3-5: J-piece — glass (tinted green via scene color bounce) ────
    let mat_j = Arc::new(Dielectric::new(1.45));
    piece(&mut world, -3.0, 3.0, 0.0,
          &[(1,0),(1,1),(0,2),(1,2)], mat_j);

    // ── Row 5-6: Z-piece — rose gold metal ───────────────────────────────
    let mat_z = Arc::new(Metal::new(Color::new(0.80, 0.55, 0.50), 0.08));
    piece(&mut world, 0.0, 5.0, 0.0,
          &[(0,1),(1,1),(1,0),(2,0)], mat_z);

    // ── Row 4-7: I-piece — glass, vertical, falling ──────────────────────
    // Tall glass column shows deep refraction
    let mat_i2 = Arc::new(Dielectric::new(1.5));
    piece(&mut world, 3.0, 4.0, 0.0,
          &[(0,0),(0,1),(0,2),(0,3)], mat_i2);

    // ── Row 7-8: O-piece — copper mirror, at top ─────────────────────────
    let mat_o2 = Arc::new(Metal::new(Color::new(0.72, 0.45, 0.20), 0.02));
    piece(&mut world, -2.0, 7.0, 0.0,
          &[(0,0),(1,0),(0,1),(1,1)], mat_o2);

    world
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32  = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 200;
    let max_depth: u32 = 50;

    let world = Arc::new(build_scene());

    // Camera: lower angle, closer, looking slightly up
    // This makes shadows visible on the floor and reflections more dramatic
    let look_from  = Point3::new(0.0, 3.5, 11.0);
    let look_at    = Point3::new(0.0, 3.5,  0.0);
    let focus_dist = (look_from - look_at).length();
    let camera = Arc::new(Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        42.0,
        aspect_ratio,
        0.06,
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