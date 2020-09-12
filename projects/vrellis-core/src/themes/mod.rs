mod builder;
use crate::ColorAverage;
pub use builder::{repack_all_theme, repack_directory};
use image::{imageops::FilterType, DynamicImage, ImageFormat, ImageOutputFormat, Rgb};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MosaicCraftThemeConfig {
    name: String,
    authors: Vec<String>,
    images_path: Vec<String>,
    images_pack: Option<String>,
    color_average: ColorAverage,
    preview: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MosaicCraftTheme {
    pub name: String,
    pub authors: Vec<String>,
    pub color_average: ColorAverage,
    pub images: Vec<MosaicCraftThemeItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MosaicCraftThemeItem {
    pub color: (f32, f32, f32),
    image: Vec<u8>,
}

impl Debug for MosaicCraftTheme {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("MosaicCraftTheme")
            .field("name", &self.name)
            .field("authors", &self.authors)
            .field("images", &self.images.len())
            .finish()
    }
}

impl Default for MosaicCraftThemeConfig {
    fn default() -> Self {
        Self {
            name: String::from("anonymous"),
            authors: vec![],
            images_path: vec![],
            images_pack: None,
            color_average: Default::default(),
            preview: None,
        }
    }
}

impl From<MosaicCraftThemeConfig> for MosaicCraftTheme {
    fn from(cfg: MosaicCraftThemeConfig) -> Self {
        Self { name: cfg.name, authors: cfg.authors, color_average: cfg.color_average, images: vec![] }
    }
}

impl MosaicCraftTheme {
    pub fn load_buildin() -> Self {
        let dump = include_bytes!("minecraft2d.mosaic-craft-theme") as &[u8];
        bincode::deserialize(dump).unwrap()
    }
}

impl MosaicCraftThemeItem {
    pub fn new(color: Rgb<u8>, image: DynamicImage) -> Self {
        let mut buf = vec![];
        image.write_to(&mut buf, ImageOutputFormat::Png).unwrap();
        unsafe {
            let r = *color.0.get_unchecked(0);
            let g = *color.0.get_unchecked(1);
            let b = *color.0.get_unchecked(2);
            Self { color: (r as f32, g as f32, b as f32), image: buf }
        }
    }
    pub fn rgb(&self) -> Rgb<u8> {
        Rgb([self.color.0 as u8, self.color.1 as u8, self.color.2 as u8])
    }
    pub fn image(&self) -> DynamicImage {
        image::load_from_memory_with_format(&self.image, ImageFormat::Png).unwrap()
    }
    pub fn resized_image(&self, size: u32) -> DynamicImage {
        self.image().resize_exact(size, size, FilterType::Nearest)
    }
    pub fn resized_item(&self, size: u32) -> Self {
        let mut buf = vec![];
        self.resized_image(size).write_to(&mut buf, ImageOutputFormat::Png).unwrap();
        Self { color: self.color, image: buf }
    }
}
