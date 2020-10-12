mod canvas;
mod errors;
mod render;
pub use crate::{
    canvas::{VrellisCanvas, MosaicCraftCanvasItem},
    errors::{VrellisError, Result},
    render::{VrellisShape, ColorMetrics},
};
use image::GenericImageView;
pub use image::{Luma, Rgb};

pub const MOSAIC_CRAFT_MAX_BLOCK_SIZE: u32 = 1024;
pub const MOSAIC_CRAFT_THEME_CONFIG_NAME: &str = "mosaic-craft-theme.json";

#[derive(Debug, Clone)]
pub struct Vrellis {
    pub color_average: VrellisShape,
    pub color_metrics: ColorMetrics,
    pub background: Option<Rgb<u8>>,
    pub grid_size: u32,
    pub magnify: f32,
}

impl Default for Vrellis {
    fn default() -> Self {
        Self {
            color_average: Default::default(),
            color_metrics: Default::default(),
            background: None,
            grid_size: 16,
            magnify: 1.0,
        }
    }
}
