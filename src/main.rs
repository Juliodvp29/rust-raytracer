use rust_raytracer::{
    app,
    geometry::{Bounded, Sphere},
    materials::{Dielectric, Lambertian, Material, Metal},
    math::{Color, Point3},
};
use std::sync::Arc;

/// Constructs the scene as a list of hittable objects.
/// Each object is heap-allocated and reference-counted so it can be
/// shared safely across rayon threads during parallel rendering.
fn build_scene() -> Vec<Arc<dyn Bounded>> {
    let mut objects: Vec<Arc<dyn Bounded>> = Vec::new();

    // Ground plane — a very large sphere acting as a flat diffuse floor
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::new(0.82, 0.82, 0.82))),
    )));

    // Central feature sphere — large glass ball (index of refraction 1.5 ≈ borosilicate glass)
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.8, 0.0),
        1.5,
        Arc::new(Dielectric::new(1.5)),
    )));

    // Left accent sphere — polished gold metal (roughness 0.0 = perfect mirror)
    objects.push(Arc::new(Sphere::new(
        Point3::new(-3.2, 0.7, 0.5),
        0.7,
        Arc::new(Metal::new(Color::new(0.83, 0.68, 0.22), 0.0)),
    )));

    // Right accent sphere — near-perfect silver metal (very slight roughness)
    objects.push(Arc::new(Sphere::new(
        Point3::new(3.2, 0.7, 0.5),
        0.7,
        Arc::new(Metal::new(Color::new(0.9, 0.9, 0.95), 0.02)),
    )));

    // Front-left mid sphere — matte red
    objects.push(Arc::new(Sphere::new(
        Point3::new(-1.5, 0.45, 2.2),
        0.45,
        Arc::new(Lambertian::new(Color::new(0.8, 0.15, 0.1))),
    )));

    // Front-right mid sphere — matte blue
    objects.push(Arc::new(Sphere::new(
        Point3::new(1.5, 0.45, 2.2),
        0.45,
        Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.8))),
    )));

    // Front-center small glass sphere (higher IOR = 1.7 for a denser look)
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 0.3, 2.8),
        0.3,
        Arc::new(Dielectric::new(1.7)),
    )));

    // Back-left sphere — warm copper-toned metal with slight roughness
    objects.push(Arc::new(Sphere::new(
        Point3::new(-2.2, 0.4, -1.8),
        0.4,
        Arc::new(Metal::new(Color::new(0.8, 0.5, 0.4), 0.1)),
    )));

    // Back-right sphere — gold-bronze metal, nearly specular
    objects.push(Arc::new(Sphere::new(
        Point3::new(2.2, 0.4, -1.8),
        0.4,
        Arc::new(Metal::new(Color::new(0.72, 0.45, 0.2), 0.05)),
    )));

    // Back-center small sphere — matte green
    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 0.3, -2.5),
        0.3,
        Arc::new(Lambertian::new(Color::new(0.1, 0.7, 0.2))),
    )));

    // Six small decorative spheres scattered around the scene.
    // Materials cycle through the palette defined below.
    let small_mats: Vec<Arc<dyn Material>> = vec![
        Arc::new(Lambertian::new(Color::new(0.9, 0.7, 0.1))), // yellow diffuse
        Arc::new(Metal::new(Color::new(0.6, 0.8, 0.9), 0.3)), // light-blue metal
        Arc::new(Dielectric::new(1.4)),                       // glass-like
        Arc::new(Lambertian::new(Color::new(0.7, 0.1, 0.7))), // purple diffuse
        Arc::new(Metal::new(Color::new(0.5, 0.9, 0.5), 0.2)), // green metal
        Arc::new(Lambertian::new(Color::new(0.9, 0.4, 0.1))), // orange diffuse
    ];

    // World-space positions for the small spheres (x, y, z)
    let positions = [
        (-1.0_f64, 0.2_f64, -0.8_f64),
        (1.0, 0.2, -0.8),
        (-0.5, 0.2, 1.5),
        (0.5, 0.2, 1.5),
        (-2.5, 0.2, 1.2),
        (2.5, 0.2, 1.2),
    ];

    for (i, (x, y, z)) in positions.iter().enumerate() {
        objects.push(Arc::new(Sphere::new(
            Point3::new(*x, *y, *z),
            0.2,
            // Wrap around the material list if there are more positions than materials
            Arc::clone(&small_mats[i % small_mats.len()]),
        )));
    }

    objects
}

fn main() {
    // Build the scene graph (BVH construction happens inside app::run)
    let objects = build_scene();
    eprintln!("Scene built with {} objects", objects.len());

    // Hand off to the windowed render loop (winit + pixels crate)
    app::run(objects);
}
