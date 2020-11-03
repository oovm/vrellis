use crate::{render::VrellisSequence, VrellisCanvas};
use image::DynamicImage;

impl VrellisCanvas {
    pub fn draw_sequence(&self, s: &VrellisSequence) -> String {
        format!("{:#?}", s);
        unimplemented!()
    }
}

impl VrellisCanvas {
    pub fn draw_svg(&self) -> String {
        unimplemented!()
    }
}

impl VrellisCanvas {
    pub fn draw_image(&self) -> DynamicImage {
        DynamicImage::ImageLumaA8(self.current_image.clone())
    }
}

impl VrellisCanvas {
    pub fn draw_canvas(&self) {
        unimplemented!()
    }
}
