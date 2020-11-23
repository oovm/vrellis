mod draw;
mod save;

use crate::{VrellisAlgorithm, VrellisPoint};
use image::{GrayAlphaImage, GrayImage, RgbImage};
use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
};

#[derive(Clone)]
pub struct VrellisCanvas {
    pub algorithm: VrellisAlgorithm,
    pub min_distance: u32,
    pub inverted_color: bool,
    pub target_image: RgbImage,
    pub current_image: GrayAlphaImage,
    pub current_composite_image: GrayImage,
    pub points: Vec<VrellisPoint>,
    pub path: Vec<u32>,
    pub path_banned: HashSet<(u32, u32)>,
    pub last_point: VrellisPoint,
    pub line_width: f32,
}

impl Debug for VrellisCanvas {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("VrellisCanvas")
            .field("width", &self.target_image.width())
            .field("height", &self.target_image.height())
            .field("path", &self.path)
            .finish()
    }
}

impl VrellisCanvas {
    pub fn steps(&mut self, n: u32) {
        for _ in 0..n {
            if let None = self.next() {
                break;
            }
        }
    }
}
