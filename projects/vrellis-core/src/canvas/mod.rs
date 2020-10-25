mod draw;
mod save;

use image::{DynamicImage, GenericImageView};
use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

#[derive(Clone)]
pub struct VrellisCanvas {
    pub size_x: u32,
    pub size_y: u32,
    pub target_image: DynamicImage,
    pub image_state: DynamicImage,
    pub state: u32,
    pub path: Vec<u32>,
    pub path_banned: HashSet<(u32, u32)>,
}

impl Debug for VrellisCanvas {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("VrellisCanvas")
            .field("target_image", &self.target_image.dimensions())
            .field("image_state", &self.image_state.dimensions())
            .field("state", &self.state)
            .field("path", &self.path)
            .finish()
    }
}
