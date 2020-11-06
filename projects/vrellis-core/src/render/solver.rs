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
        let old = *self.path.last().unwrap();

        let mut selected = 0;
        for new in 0..self.points.len() {
            if !self.should_skip(new) {
                unsafe {
                     let this_line = self.points.get_unchecked(new)
                };


                self.algorithm.line_score(self)
            }
        }
        // No legal line segment, no line is selected
        if selected == 0 {
            return None
        }

        swap(&mut self.current_image, &mut out);

        self.path.push(selected);
        self.path_banned.insert((selected, old));
        self.path_banned.insert((old, selected));
        return Some(out);
    }
}

impl VrellisCanvas {
    fn should_skip(&self, this: usize) -> bool {
        let old = *self.path.last().unwrap();
        let this = this as u32;
        if old == this {
            true
        }
        else if old - this <= self.min_distance {
            true
        }
        else if self.path_banned.contains(&(old, this)) {
            true
        }
        else {
            false
        }
    }
}
