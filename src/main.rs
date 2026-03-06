use rust_raytracer::{
    math::{Vec3, Color, Point3},
    core::Ray,
    geometry::{Hittable, Sphere},
    scene::World,
    render::Camera,
    utils::to_rgb,
};


fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        return (rec.normal + Vec3::one()) * 0.5;
    }

    let unit = ray.direction.normalize();
    let t = 0.5 * (unit.y + 1.0); 
    Color::one() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}


fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 1; 

    let mut world = World::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))); 

    let camera = Camera::new(aspect_ratio, 2.0, 1.0);

    let mut pixels: Vec<(u8, u8, u8)> = Vec::with_capacity((image_width * image_height) as usize);

    eprintln!("Rendering {}x{} image...", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray = camera.get_ray(u, v);
            let color = ray_color(&ray, &world);

            pixels.push(to_rgb(color, samples_per_pixel));
        }
    }

    println!("P3\n{} {}\n255", image_width, image_height);
    for (r, g, b) in &pixels {
        println!("{} {} {}", r, g, b);
    }

    eprintln!("Done! Redirect stdout to a .ppm file to view the image.");
}
