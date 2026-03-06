use rust_raytracer::{
    app,
    geometry::{Bounded, Sphere},
    materials::{Dielectric, Lambertian, Material, Metal},
    math::{Color, Point3},
};
use std::sync::Arc;

fn build_scene() -> Vec<Arc<dyn Bounded>> {
    let mut objects: Vec<Arc<dyn Bounded>> = Vec::new();

    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::new(0.82, 0.82, 0.82))),
    )));

    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.8, 0.0),
        1.5,
        Arc::new(Dielectric::new(1.5)),
    )));

    objects.push(Arc::new(Sphere::new(
        Point3::new(-3.2, 0.7, 0.5),
        0.7,
        Arc::new(Metal::new(Color::new(0.83, 0.68, 0.22), 0.0)),
    )));

    objects.push(Arc::new(Sphere::new(
        Point3::new(3.2, 0.7, 0.5),
        0.7,
        Arc::new(Metal::new(Color::new(0.9, 0.9, 0.95), 0.02)),
    )));

    objects.push(Arc::new(Sphere::new(
        Point3::new(-1.5, 0.45, 2.2),
        0.45,
        Arc::new(Lambertian::new(Color::new(0.8, 0.15, 0.1))),
    )));

    objects.push(Arc::new(Sphere::new(
        Point3::new(1.5, 0.45, 2.2),
        0.45,
        Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.8))),
    )));

    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 0.3, 2.8),
        0.3,
        Arc::new(Dielectric::new(1.7)),
    )));

    objects.push(Arc::new(Sphere::new(
        Point3::new(-2.2, 0.4, -1.8),
        0.4,
        Arc::new(Metal::new(Color::new(0.8, 0.5, 0.4), 0.1)),
    )));

    objects.push(Arc::new(Sphere::new(
        Point3::new(2.2, 0.4, -1.8),
        0.4,
        Arc::new(Metal::new(Color::new(0.72, 0.45, 0.2), 0.05)),
    )));

    objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 0.3, -2.5),
        0.3,
        Arc::new(Lambertian::new(Color::new(0.1, 0.7, 0.2))),
    )));

    let small_mats: Vec<Arc<dyn Material>> = vec![
        Arc::new(Lambertian::new(Color::new(0.9, 0.7, 0.1))),
        Arc::new(Metal::new(Color::new(0.6, 0.8, 0.9), 0.3)),
        Arc::new(Dielectric::new(1.4)),
        Arc::new(Lambertian::new(Color::new(0.7, 0.1, 0.7))),
        Arc::new(Metal::new(Color::new(0.5, 0.9, 0.5), 0.2)),
        Arc::new(Lambertian::new(Color::new(0.9, 0.4, 0.1))),
    ];

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
            Arc::clone(&small_mats[i % small_mats.len()]),
        )));
    }

    objects
}

fn main() {
    let objects = build_scene();
    eprintln!("Scene built with {} objects", objects.len());
    app::run(objects);
}
