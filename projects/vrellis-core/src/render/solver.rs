use crate::{Result, Vrellis, VrellisCanvas};
use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgb};
use imageproc::drawing::draw_antialiased_line_segment;
use std::{io::Cursor, mem::swap, path::Path};

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
            min_distance: 0,
            target_image: img.clone(),
            current_image: img.clone(),
            current_composite_image: img.clone().to_luma(),
            points: vec![],
            path: vec![],
            path_banned: Default::default(),
        };
        unimplemented!()
    }
}

impl Iterator for VrellisCanvas {
    type Item = DynamicImage;
    fn next(&mut self) -> Option<Self::Item> {
        let mut out = DynamicImage::new_rgb8(100, 100);

        let new = 2;

        swap(&mut self.current_image, &mut out);
        let old = *self.path.last().unwrap();
        self.path.push(new);
        self.path_banned.insert((new, old));
        self.path_banned.insert((old, new));
        return Some(out);
    }
}
