use crate::{
    themes::MosaicCraftThemeConfig, MosaicCraftTheme, MosaicCraftThemeItem, Result, MOSAIC_CRAFT_MAX_BLOCK_SIZE,
    MOSAIC_CRAFT_THEME_CONFIG_NAME,
};
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageOutputFormat};
use std::{
    fs::{self, read_to_string},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn repack_directory(config_from: impl AsRef<Path>, pack_to: impl AsRef<Path>) -> Result<MosaicCraftTheme> {
    let config_file = config_from.as_ref().join(MOSAIC_CRAFT_THEME_CONFIG_NAME);
    let mut config = MosaicCraftThemeConfig::try_parse_config(&config_file);
    let mut theme = MosaicCraftTheme::from(config.clone());
    for entry in std::fs::read_dir(config_from)?.filter_map(|e| e.ok()) {
        if let Some((img, name)) = MosaicCraftThemeConfig::try_parse_png(entry) {
            config.images_path.push(name);
            theme.push_image(img);
        }
    }
    fs::write(&config_file, serde_json::to_string_pretty(&config)?)?;
    fs::write(pack_to, bincode::serialize(&theme)?)?;
    return Ok(theme);
}

pub fn repack_all_theme(from_dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> Result<()> {
    for entry in WalkDir::new(from_dir).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        if f_name == MOSAIC_CRAFT_THEME_CONFIG_NAME {
            let dir_name = entry.path().parent().unwrap().to_path_buf();
            let out_name = format!("{}.mosaic-craft-theme", dir_name.to_string_lossy());
            let out_file = out_dir.as_ref().join(&out_name);
            match repack_directory(dir_name, out_file) {
                Ok(o) => println!("Packing {} images to {}", o.images.len(), out_name),
                Err(e) => println!("{:?}", e),
            }
        }
    }
    Ok(())
}

impl MosaicCraftTheme {
    fn push_image(&mut self, img: DynamicImage) {
        let checked = check_image_size(img);
        let mean = self.color_average.mean(&checked);
        let mut buf = vec![];
        checked.write_to(&mut buf, ImageOutputFormat::Png).unwrap();
        self.images.push(MosaicCraftThemeItem { color: mean, image: buf });
    }
}

impl MosaicCraftThemeConfig {
    fn try_parse_config(file: &PathBuf) -> Self {
        let name = file.parent().and_then(|e| e.file_name()).unwrap().to_string_lossy().to_string();
        let raw = read_to_string(file).unwrap_or(String::from("|"));
        match serde_json::from_str::<Self>(&raw) {
            Ok(o) => o,
            Err(_) => Self { name, ..Default::default() },
        }
    }
    fn try_parse_png(file: std::fs::DirEntry) -> Option<(DynamicImage, String)> {
        match image::open(file.path()) {
            Ok(o) => {
                let name = file.file_name().to_string_lossy().to_string();
                Some((o, name))
            }
            Err(_) => None,
        }
    }
}

#[rustfmt::skip]
fn check_image_size(image: DynamicImage) -> DynamicImage {
    if image.width() > MOSAIC_CRAFT_MAX_BLOCK_SIZE
    || image.height() > MOSAIC_CRAFT_MAX_BLOCK_SIZE
    || image.width() != image.height()
    {
        let size = MOSAIC_CRAFT_MAX_BLOCK_SIZE.min(image.width()).min(image.height());
        image.resize_exact(size, size, FilterType::Nearest)
    }
    else {
        return image;
    }
}
