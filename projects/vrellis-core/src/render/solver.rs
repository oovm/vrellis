use crate::{Result, Vrellis, VrellisCanvas};
use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgb};
use std::{io::Cursor, path::Path};
use std::mem::swap;

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
        VrellisCanvas {
            size_x: img.width(),
            size_y: img.height(),
            target_image: img.clone(),
            image_state: img.clone(),
            path: vec![],
            path_banned: Default::default()
        };
        unimplemented!()
    }
}

impl Iterator for VrellisCanvas {
    type Item = DynamicImage;
    fn next(&mut self) -> Option<Self::Item> {

        let mut out = DynamicImage::new_rgb8(100,100);

        let new = 2;



        swap(&mut self.image_state, &mut out);
        let old = *self.path.last().unwrap();
        self.path.push(new);
        self.path_banned.insert((new, old));
        self.path_banned.insert((old, new));
        return Some(out)
    }
}