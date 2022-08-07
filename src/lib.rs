pub mod camera;
pub mod color;
pub mod hittable;
pub mod ray;
pub mod vec3;

/* Re-exports */
pub use std::f64::INFINITY;
pub use std::f64::consts::PI;

/* Utility functions */
use rand::random;

use image::RgbImage;

use color::{Color, into_pixel};

/// Writes a pixel into img
pub fn write_pixel(img: &mut RgbImage, pixel_color: &Color, x: u32, y: u32, n_samples: u32) {
    let pixel_color = pixel_color / (n_samples as f64);
    img.put_pixel(x, y, into_pixel(&pixel_color));
}

/// Clamps x in [min, max]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.min(max).max(min)
}

/// Returns a random f64 in [0, 1)
pub fn random_f64() -> f64 {
    random()
}

/// Returns a random f64 in [min, max)
pub fn random_f64_in(min: f64, max: f64) -> f64 {
    min + (max - min)*random_f64()
}
