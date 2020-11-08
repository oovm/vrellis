use crate::{Result, Vrellis, VrellisCanvas, VrellisPoint};
use image::{io::Reader, DynamicImage, GenericImageView, GrayAlphaImage, ImageBuffer, Rgb};
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
            inverted_color: self.inverted_color,
            target_image: img.to_rgb(),
            current_image: canvas.to_luma_alpha(),
            current_composite_image: img.to_luma(),
            points: points_sample,
            path: vec![initial_point.n],
            path_banned: Default::default(),
            last_point: initial_point.clone(),
        }
    }
}

impl Iterator for VrellisCanvas {
    type Item = DynamicImage;
    fn next(&mut self) -> Option<Self::Item> {
        let mut out = DynamicImage::new_rgb8(100, 100);
        let mut selected = None;
        let mut max_score = 0.0;
        for new in self.points.iter() {
            if self.should_skip(new) {
                continue;
            }
            let score = self.algorithm.line_score(
                &self.current_composite_image,
                self.last_point.x,
                self.last_point.y,
                new.x,
                new.y,
                self.inverted_color,
            );
            if score > max_score {
                max_score = score;
                selected = Some(new)
            }
        }
        // No legal line segment, no line is selected
        if let None = selected {
            return None;
        };
        let selected = selected.unwrap();
        self.algorithm.draw_line(
            &mut self.current_composite_image,
            self.last_point.x,
            self.last_point.y,
            new.x,
            new.y,
            self.inverted_color,
        );
        self.draw_canvas_line(&mut self.current_image, self.last_point.x, self.last_point.y, new.x, new.y, self.inverted_color);
        let new = selected.unwrap().n;
        let old = *self.path.last().unwrap();
        self.path.push(new);
        self.path_banned.insert((new, old));
        self.path_banned.insert((old, new));
        return Some(DynamicImage::ImageLumaA8(self.current_image.clone()));
    }
}

impl VrellisCanvas {
    fn should_skip(&self, this: &VrellisPoint) -> bool {
        let old = *self.path.last().unwrap();
        let this = this.n;
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
