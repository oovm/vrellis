use crate::{Result, VrellisCanvas};
use image::ImageFormat;
use std::{fs, path::Path};

impl VrellisCanvas {
    pub fn save_image_steps(&self, dir: impl AsRef<Path>) -> Result<()> {
        let _ = dir;
        unimplemented!()
    }
    pub fn save_image(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(self.draw_image().save_with_format(path, ImageFormat::Png)?)
    }
}

impl VrellisCanvas {
    pub fn save_svg_steps(&self, dir: impl AsRef<Path>) -> Result<()> {
        let _ = dir;
        unimplemented!()
    }
    pub fn save_svg(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(fs::write(path, self.draw_svg().as_bytes())?)
    }
}

impl VrellisCanvas {
    pub fn save_sequence(&self, _: impl AsRef<Path>) -> Result<()> {
        unimplemented!()
    }
}
