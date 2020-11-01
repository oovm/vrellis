mod draw;
mod save;

use image::{DynamicImage, GenericImageView, GrayImage};
use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
};
use crate::VrellisPoint;

#[derive(Clone)]
pub struct VrellisCanvas {
    pub size_x: u32,
    pub size_y: u32,
    pub min_distance: u32,
    pub target_image: DynamicImage,
    pub current_image: DynamicImage,
    pub current_composite_image: GrayImage,
    pub points: Vec<VrellisPoint>,
    pub path: Vec<u32>,
    pub path_banned: HashSet<(u32, u32)>,
}

impl Debug for VrellisCanvas {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("VrellisCanvas")
            .field("size_x", &self.size_x)
            .field("size_y", &self.size_y)
            .field("path", &self.path)
            .finish()
    }
}
