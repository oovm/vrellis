mod canvas;
mod errors;
mod render;
pub use crate::{
    canvas::{MosaicCraftCanvasItem, VrellisCanvas},
    errors::{Result, VrellisError},
    render::{VrellisColorMode, VrellisShape},
};
use image::GenericImageView;
pub use image::{Luma, Rgb};

pub const MOSAIC_CRAFT_MAX_BLOCK_SIZE: u32 = 1024;
pub const MOSAIC_CRAFT_THEME_CONFIG_NAME: &str = "mosaic-craft-theme.json";

#[derive(Debug, Clone)]
pub struct Vrellis {
    pub convex_shape: VrellisShape,
    pub color_mode: VrellisColorMode,
    pub background: Option<Rgb<u8>>,
    pub grid_size: u32,
    pub magnify: f32,
}

impl Default for Vrellis {
    fn default() -> Self {
        Self {
            convex_shape: Default::default(),
            color_metrics: Default::default(),
            background: None,
            grid_size: 16,
            magnify: 1.0,
        }
    }
}
