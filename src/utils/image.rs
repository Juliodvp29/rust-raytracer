use crate::math::Color;

pub fn to_rgb(color: Color, samples_per_pixel: u32) -> (u8, u8, u8) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = (color.x * scale).sqrt().clamp(0.0, 0.999);
    let g = (color.y * scale).sqrt().clamp(0.0, 0.999);
    let b = (color.z * scale).sqrt().clamp(0.0, 0.999);

    (
        (r * 256.0) as u8,
        (g * 256.0) as u8,
        (b * 256.0) as u8,
    )
}

pub fn write_ppm(pixels: &[(u8, u8, u8)], width: u32, height: u32) {
    println!("P3\n{} {}\n255", width, height);
    for (r, g, b) in pixels {
        println!("{} {} {}", r, g, b);
    }
}
