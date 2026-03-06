use std::sync::Arc;
use rayon::prelude::*;
use rust_raytracer::{
    math::{Vec3, Color, Point3, random_f64},
    core::Ray,
    geometry::{Hittable, Aabb, Bounded},
    scene::World,
    render::Camera,
    materials::{Lambertian, Metal, Dielectric, Material},
    utils::to_rgb,
};

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
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


fn build_scene() -> Vec<Arc<dyn Bounded>> {
    let mut objects: Vec<Arc<dyn Bounded>> = Vec::new();

    // ── Floor ────────────────────────────────────────────────────────────
    let mat_floor = Arc::new(Lambertian::new(Color::new(0.82, 0.82, 0.82)));
    objects.push(Arc::new(Aabb::new(
        Point3::new(-7.0, -0.15, -4.0),
        Point3::new( 7.0,  0.0,   6.0),
        mat_floor,
    )));

    // ── Back wall ─────────────────────────────────────────────────────────
    let mat_wall = Arc::new(Lambertian::new(Color::new(0.88, 0.86, 0.82)));
    objects.push(Arc::new(Aabb::new(
        Point3::new(-7.0, 0.0, -4.1),
        Point3::new( 7.0, 14.0, -4.0),
        mat_wall,
    )));

    // ── Left wall ─────────────────────────────────────────────────────────
    let mat_left: Arc<dyn Material> =
        Arc::new(Lambertian::new(Color::new(0.78, 0.82, 0.90)));
    objects.push(Arc::new(Aabb::new(
        Point3::new(-5.1, 0.0, -4.0),
        Point3::new(-5.0, 14.0, 6.0),
        Arc::clone(&mat_left) as Arc<dyn Material>,
    )));

    // ── Right wall ────────────────────────────────────────────────────────
    let mat_right: Arc<dyn Material> =
        Arc::new(Lambertian::new(Color::new(0.90, 0.82, 0.78)));
    objects.push(Arc::new(Aabb::new(
        Point3::new(5.0, 0.0, -4.0),
        Point3::new(5.1, 14.0, 6.0),
        Arc::clone(&mat_right) as Arc<dyn Material>,
    )));

    // ── Tetris pieces (same as before, but push to objects vec) ──────────
    let push_piece = |objects: &mut Vec<Arc<dyn Bounded>>,
                      base_x: f64, base_y: f64, base_z: f64,
                      blocks: &[(i32,i32)],
                      mat: Arc<dyn Material>| {
        let s = 0.88_f64;
        for (dx, dy) in blocks {
            let x = base_x + *dx as f64;
            let y = base_y + *dy as f64;
            objects.push(Arc::new(Aabb::new(
                Point3::new(x,     y,     base_z    ),
                Point3::new(x + s, y + s, base_z + s),
                Arc::clone(&mat),
            )));
        }
    };

    push_piece(&mut objects, -2.0, 0.0, 0.0,
        &[(0,0),(1,0),(2,0),(3,0)],
        Arc::new(Metal::new(Color::new(0.95, 0.95, 0.98), 0.0)));

    push_piece(&mut objects, 2.0, 0.0, 0.0,
        &[(0,0),(1,0),(0,1),(1,1)],
        Arc::new(Metal::new(Color::new(0.83, 0.68, 0.22), 0.04)));

    push_piece(&mut objects, -4.0, 1.0, 0.0,
        &[(0,0),(1,0),(2,0),(1,1)],
        Arc::new(Dielectric::new(1.5)));

    push_piece(&mut objects, -1.0, 2.0, 0.0,
        &[(0,0),(1,0),(1,1),(2,1)],
        Arc::new(Lambertian::new(Color::new(0.05, 0.75, 0.85))));

    push_piece(&mut objects, 2.0, 2.0, 0.0,
        &[(0,0),(0,1),(0,2),(1,0)],
        Arc::new(Metal::new(Color::new(0.70, 0.72, 0.75), 0.12)));

    push_piece(&mut objects, -3.0, 3.0, 0.0,
        &[(1,0),(1,1),(0,2),(1,2)],
        Arc::new(Dielectric::new(1.45)));

    push_piece(&mut objects, 0.0, 5.0, 0.0,
        &[(0,1),(1,1),(1,0),(2,0)],
        Arc::new(Metal::new(Color::new(0.80, 0.55, 0.50), 0.08)));

    push_piece(&mut objects, 3.0, 4.0, 0.0,
        &[(0,0),(0,1),(0,2),(0,3)],
        Arc::new(Dielectric::new(1.5)));

    push_piece(&mut objects, -2.0, 7.0, 0.0,
        &[(0,0),(1,0),(0,1),(1,1)],
        Arc::new(Metal::new(Color::new(0.72, 0.45, 0.20), 0.02)));

    objects
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32  = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 200;
    let max_depth: u32 = 50;

    // Build BVH — O(log n) traversal instead of O(n)
    let objects = build_scene();
    let object_count = objects.len();
    let bvh = Arc::new(World::build_bvh(objects));
    eprintln!("BVH built over {} objects", object_count);

    let look_from  = Point3::new(0.0, 3.5, 11.0);
    let look_at    = Point3::new(0.0, 3.5,  0.0);
    let focus_dist = (look_from - look_at).length();
    let camera = Arc::new(Camera::new(
        look_from, look_at,
        Vec3::new(0.0, 1.0, 0.0),
        42.0, aspect_ratio, 0.06, focus_dist,
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
                color += ray_color(&camera.get_ray(u, v), bvh.as_ref(), max_depth);
            }
            to_rgb(color, samples_per_pixel)
        })
        .collect();

    rust_raytracer::utils::save_png(&pixels, image_width, image_height, "output.png");
    eprintln!("Done! Saved to output.png");
}