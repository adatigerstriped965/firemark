use crate::config::types::WatermarkConfig;
use crate::error::Result;
use crate::render::canvas::Canvas;

pub trait WatermarkRenderer: Send + Sync {
    fn render(&self, config: &WatermarkConfig, width: u32, height: u32) -> Result<Canvas>;
}
