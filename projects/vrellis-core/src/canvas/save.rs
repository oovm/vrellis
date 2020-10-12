use crate::{VrellisCanvas, Result};
use image::ImageFormat;
use std::path::Path;

impl VrellisCanvas {
    pub fn save_image(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(self.draw_image().save_with_format(path, ImageFormat::Png)?)
    }
}
