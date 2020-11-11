use crate::VrellisCanvas;
use image::DynamicImage;
use itertools::Itertools;

impl VrellisCanvas {
    pub fn draw_svg(&self) -> String {
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {} {}">{}</svg>"#,
            self.current_image.width(),
            self.current_image.height(),
            format!(
                r#"<polyline points={:?} style="fill:none;stroke:black;stroke-width:1"/>"#,
                self.take_points(self.path.len())
            )
        )
    }

}

impl VrellisCanvas {
    pub fn draw_image(&self) -> DynamicImage {
        unimplemented!()
    }
}

impl VrellisCanvas {
    pub fn draw_canvas(&self) {
        unimplemented!()
    }
}
