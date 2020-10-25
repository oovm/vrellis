use crate::{Result, Vrellis, VrellisCanvas};
use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgb};
use itertools::Itertools;
use std::{io::Cursor, path::Path, rc::Rc};

impl Vrellis {
    pub fn render_path(&self, path: impl AsRef<Path>) -> Result<VrellisCanvas> {
        let img = Reader::open(path)?.decode()?;
        Ok(self.render(img))
    }
    pub fn render_bytes(&self, bytes: &[u8]) -> Result<VrellisCanvas> {
        let img = Reader::new(Cursor::new(bytes)).decode()?;
        Ok(self.render(img))
    }

    pub fn render(&self, img: DynamicImage) -> VrellisCanvas {
        unimplemented!()
    }
}
