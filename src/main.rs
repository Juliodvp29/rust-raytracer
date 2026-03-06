use rust_raytracer::{
    math::{Vec3, Color, Point3},
    core::Ray,
    geometry::{Hittable, Sphere},
    scene::World,
    render::Camera,
    utils::to_rgb,
};

/// Determines the color of a ray by checking if it hits anything in the world.
/// If no hit occurs, it returns a sky gradient blending from white (bottom) to light blue (top).
fn ray_color(ray: &Ray, world: &World) -> Color {
    // Check if the ray intersects any object; t_min=0.001 avoids self-intersection (shadow acne)
    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        // Map the surface normal (range -1..1) to a visible color (range 0..1)
        return (rec.normal + Vec3::one()) * 0.5;
    }

    // No hit: compute the sky gradient based on the ray's vertical direction
    let unit = ray.direction.normalize();
    // t=0 → bottom (white), t=1 → top (blue). Linear blend (lerp) between the two.
    let t = 0.5 * (unit.y + 1.0); 
    Color::one() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}


fn main() {
    // --- Image dimensions ---
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    // Height is derived from width so the aspect ratio is always respected
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    // Number of random rays per pixel for anti-aliasing (1 = no AA yet)
    let samples_per_pixel: u32 = 1; 

    // --- Build the scene ---
    let mut world = World::new();
    // Small foreground sphere centered in the view
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    // Large sphere acting as the ground plane (radius so big it looks flat)
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))); 

    // --- Set up the camera ---
    // viewport_height=2.0 and focal_length=1.0 are classic defaults for a simple pinhole camera
    let camera = Camera::new(aspect_ratio, 2.0, 1.0);

    // Pre-allocate pixel storage to avoid repeated heap allocations inside the loop
    let mut pixels: Vec<(u8, u8, u8)> = Vec::with_capacity((image_width * image_height) as usize);

    eprintln!("Rendering {}x{} image...", image_width, image_height);

    // Iterate rows from bottom to top (j goes high → low) to match PPM's top-down order
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            // UV coordinates normalize pixel position to [0,1] range for the camera
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray = camera.get_ray(u, v);
            let color = ray_color(&ray, &world);

            // Convert the floating-point color to 8-bit RGB, applying gamma correction
            pixels.push(to_rgb(color, samples_per_pixel));
        }
    }

    // Write the PPM header, then each pixel's RGB values to stdout
    println!("P3\n{} {}\n255", image_width, image_height);
    for (r, g, b) in &pixels {
        println!("{} {} {}", r, g, b);
    }

    eprintln!("Done! Redirect stdout to a .ppm file to view the image.");
}
