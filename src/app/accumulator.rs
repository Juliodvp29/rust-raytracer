use crate::math::Color;

/// Accumulates multiple render samples per pixel and averages them.
///
/// Each call to `add_sample` adds one full-frame sample.
/// `to_rgba` converts the running average to gamma-corrected RGBA bytes
/// ready for the `pixels` crate framebuffer.
pub struct Accumulator {
    pub width:   u32,
    pub height:  u32,
    /// Running sum of all color samples per pixel (linear space, f64)
    buffer: Vec<Color>,
    /// How many samples have been accumulated so far
    pub sample_count: u32,
}

impl Accumulator {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            buffer: vec![Color::zero(); size],
            sample_count: 0,
        }
    }

    /// Adds one full-frame sample (output of Renderer::render_sample) to the buffer.
    pub fn add_sample(&mut self, sample: &[Color]) {
        for (acc, &s) in self.buffer.iter_mut().zip(sample.iter()) {
            *acc += s;
        }
        self.sample_count += 1;
    }

    /// Resets the accumulator (called when the camera moves).
    pub fn reset(&mut self) {
        for c in self.buffer.iter_mut() {
            *c = Color::zero();
        }
        self.sample_count = 0;
    }

    /// Writes the averaged, gamma-corrected pixels into an RGBA byte slice
    /// (format expected by the `pixels` crate: [R, G, B, A, R, G, B, A, ...]).
    pub fn to_rgba(&self, out: &mut [u8]) {
        let scale = if self.sample_count == 0 {
            1.0
        } else {
            1.0 / self.sample_count as f64
        };

        for (i, color) in self.buffer.iter().enumerate() {
            // Average + gamma-2 correction (sqrt)
            let r = (color.x * scale).sqrt().clamp(0.0, 0.999);
            let g = (color.y * scale).sqrt().clamp(0.0, 0.999);
            let b = (color.z * scale).sqrt().clamp(0.0, 0.999);

            let base = i * 4;
            out[base    ] = (r * 255.999) as u8;
            out[base + 1] = (g * 255.999) as u8;
            out[base + 2] = (b * 255.999) as u8;
            out[base + 3] = 255; // alpha always opaque
        }
    }
}