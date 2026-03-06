use crate::math::Color;
use image::{ImageBuffer, RgbImage};

/// Converts a floating-point HDR color to 8-bit sRGB, accounting for multi-sample averaging
/// and gamma correction.
pub fn to_rgb(color: Color, samples_per_pixel: u32) -> (u8, u8, u8) {
    // Average sample contributions
    let scale = 1.0 / samples_per_pixel as f64;

    // sqrt applies gamma-2 correction; clamp prevents going out of [0, 255] range
    let r = (color.x * scale).sqrt().clamp(0.0, 0.999);
    let g = (color.y * scale).sqrt().clamp(0.0, 0.999);
    let b = (color.z * scale).sqrt().clamp(0.0, 0.999);

    (
        (r * 256.0) as u8,
        (g * 256.0) as u8,
        (b * 256.0) as u8,
    )
}

/// Writes an entire pixel buffer to stdout in PPM (Portable Pixmap) format.
pub fn write_ppm(pixels: &[(u8, u8, u8)], width: u32, height: u32) {
    println!("P3\n{} {}\n255", width, height);
    for (r, g, b) in pixels {
        println!("{} {} {}", r, g, b);
    }
}

/// Saves the pixel buffer as a PNG file.
pub fn save_png(pixels: &[(u8, u8, u8)], width: u32, height: u32, path: &str) {
    let mut img: RgbImage = ImageBuffer::new(width, height);
    for (i, (r, g, b)) in pixels.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        img.put_pixel(x, y, image::Rgb([*r, *g, *b]));
    }
    img.save(path).expect("Failed to save PNG image");
}
