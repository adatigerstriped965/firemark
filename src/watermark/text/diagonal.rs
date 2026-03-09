use crate::config::types::WatermarkConfig;
use crate::error::Result;
use crate::render::canvas::Canvas;
use crate::watermark::background::render_text_background;
use crate::watermark::renderer::WatermarkRenderer;

/// Full-page repeating diagonal text grid — the classic security watermark
/// seen on bank statements, legal documents, and confidential papers.
///
/// Main text and secondary text alternate on every other row, creating dense
/// intercalated coverage that is impossible to crop away.
pub struct DiagonalRenderer;

impl WatermarkRenderer for DiagonalRenderer {
    fn render(&self, config: &WatermarkConfig, width: u32, height: u32) -> Result<Canvas> {
        render_text_background(config, width, height, 1.0)
    }
}
