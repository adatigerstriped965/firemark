use image::Rgba;
use imageproc::geometric_transformations::{self, Interpolation};

use crate::render::canvas::Canvas;

/// Rotate the canvas contents by `angle_degrees` around the center.
///
/// Transparent pixels are used for areas outside the original image.
pub fn rotate_canvas(canvas: &Canvas, angle_degrees: f32) -> Canvas {
    let radians = angle_degrees.to_radians();
    let default = Rgba([0u8, 0, 0, 0]);
    let rotated = geometric_transformations::rotate_about_center(
        canvas.image(),
        radians,
        Interpolation::Bilinear,
        default,
    );
    Canvas::from_image(rotated)
}

/// Scale the canvas by `factor` using Lanczos3 resampling.
///
/// A factor of `1.0` returns the original size; `0.5` halves each dimension.
pub fn scale_canvas(canvas: &Canvas, factor: f32) -> Canvas {
    let new_w = ((canvas.width() as f32) * factor).round().max(1.0) as u32;
    let new_h = ((canvas.height() as f32) * factor).round().max(1.0) as u32;
    let resized = image::imageops::resize(
        canvas.image(),
        new_w,
        new_h,
        image::imageops::FilterType::Lanczos3,
    );
    Canvas::from_image(resized)
}
