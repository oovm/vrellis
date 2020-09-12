mod canvas;
mod errors;
mod render;
mod themes;
pub use crate::{
    canvas::{MosaicCraftCanvas, MosaicCraftCanvasItem},
    errors::{MosaicCraftError, Result},
    render::{ColorAverage, ColorMetrics},
    themes::{repack_all_theme, repack_directory, MosaicCraftTheme, MosaicCraftThemeItem},
};
use image::GenericImageView;
pub use image::{Luma, Rgb};

pub const MOSAIC_CRAFT_MAX_BLOCK_SIZE: u32 = 1024;
pub const MOSAIC_CRAFT_THEME_CONFIG_NAME: &str = "mosaic-craft-theme.json";

#[derive(Debug, Clone)]
pub struct MosaicCraft {
    pub color_average: ColorAverage,
    pub color_metrics: ColorMetrics,
    pub theme: MosaicCraftTheme,
    pub background: Option<Rgb<u8>>,
    pub grid_size: u32,
    pub magnify: f32,
}

impl Default for MosaicCraft {
    fn default() -> Self {
        let theme = MosaicCraftTheme::load_buildin();
        Self {
            color_average: Default::default(),
            color_metrics: Default::default(),
            theme,
            background: None,
            grid_size: 16,
            magnify: 1.0,
        }
    }
}

impl From<MosaicCraftTheme> for MosaicCraft {
    fn from(theme: MosaicCraftTheme) -> Self {
        let min_size = theme.images.iter().map(|item| item.image().width()).min().unwrap();
        Self { color_average: theme.color_average, theme, grid_size: min_size, ..Self::default() }
    }
}
