use image::Rgba;
use qrcode::QrCode;

use crate::error::{FiremarkError, Result};
use crate::render::canvas::Canvas;

/// Generate a QR code as a `Canvas` with the given pixel `size` and foreground `color`.
///
/// The QR modules are scaled to fill the requested size as closely as possible.
/// Background pixels are left fully transparent.
pub fn generate_qr(data: &str, size: u32, color: Rgba<u8>) -> Result<Canvas> {
    let code = QrCode::new(data.as_bytes())
        .map_err(|e| FiremarkError::Other(format!("QR code generation failed: {e}")))?;

    let modules = code.to_colors();
    let module_count = code.width() as u32;

    // Scale factor: how many pixels per QR module.
    let scale = size / module_count;
    let scale = scale.max(1);
    let actual_size = scale * module_count;

    let mut canvas = Canvas::new(actual_size, actual_size);

    for (idx, &module_color) in modules.iter().enumerate() {
        let mx = (idx as u32) % module_count;
        let my = (idx as u32) / module_count;

        if module_color == qrcode::Color::Dark {
            let px = mx * scale;
            let py = my * scale;
            for dy in 0..scale {
                for dx in 0..scale {
                    canvas.set_pixel((px + dx) as i32, (py + dy) as i32, color);
                }
            }
        }
    }

    Ok(canvas)
}
