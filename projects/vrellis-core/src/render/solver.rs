use crate::{Result, Vrellis, VrellisCanvas, VrellisPoint};
use image::{io::Reader, DynamicImage, GenericImageView};
use std::{io::Cursor, path::Path};

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
        let points_sample = self.convex_shape.sample(self.points, img.width(), img.height());
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
            points: points_sample.clone(),
            path: vec![initial_point.n],
            path_banned: Default::default(),
            last_point: initial_point.clone(),
        }
    }
}

impl Iterator for VrellisCanvas {
    type Item = DynamicImage;
    fn next(&mut self) -> Option<Self::Item> {
        let mut selected = None;
        let mut max_score = 0.0;
        for point in self.points.iter() {
            if self.should_skip(point) {
                continue;
            }
            let score = self.algorithm.line_score(
                &self.current_composite_image,
                self.last_point.x,
                self.last_point.y,
                point.x,
                point.y,
                self.inverted_color,
            );
            if score > max_score {
                max_score = score;
                selected = Some(point)
            }
        }
        // No legal line segment, no line is selected
        if let None = selected {
            return None;
        };
        let selected = selected.unwrap().clone();
        self.algorithm.draw_line(
            &mut self.current_composite_image,
            self.last_point.x,
            self.last_point.y,
            selected.x,
            selected.y,
            self.inverted_color,
        );
        self.draw_canvas_line(self.last_point.x, self.last_point.y, selected.x, selected.y, self.inverted_color);
        let new = selected.n;
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
        else if old > this && old - this <= self.min_distance {
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
