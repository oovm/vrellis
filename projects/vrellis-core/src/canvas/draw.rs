use crate::VrellisCanvas;
use image::{DynamicImage, GenericImage};

impl VrellisCanvas {
    pub fn draw_sequence(&self) -> String {
        format!("{:#?}", self.path)
    }
}

impl VrellisCanvas {
    pub fn draw_svg(&self) -> String {
        unimplemented!()
    }
}

impl VrellisCanvas {
    pub fn draw_image(&self) -> DynamicImage {
        unimplemented!()
    }
}

impl VrellisCanvas {
    pub fn draw_canvas(&self) {}
}
