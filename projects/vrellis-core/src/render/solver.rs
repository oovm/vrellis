use crate::{Result, Vrellis, VrellisCanvas};
use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgb};
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
        let canvas = match self.inverted_color {
            true => DynamicImage::new_luma_a8(img.width(), img.height()),
            false => DynamicImage::new_luma_a8(img.width(), img.height()),
        };
        let points_sample = self.convex_shape.sample(img.width(), img.height(), self.points);
        let initial_point = match self.inverted_color {
            true => points_sample.iter().min_by_key(|p| p.n).unwrap(),
            false => points_sample.iter().min_by_key(|p| p.n).unwrap(),
        };
        VrellisCanvas {
            algorithm: self.algorithm,
            min_distance: self.min_distance,
            target_image: img.to_rgb(),
            current_image: canvas.to_luma_alpha(),
            current_composite_image: img.to_luma(),
            points: points_sample,
            path: vec![initial_point.n],
            path_banned: Default::default(),
        }
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
