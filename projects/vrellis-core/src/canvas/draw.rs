use crate::VrellisCanvas;
use image::{DynamicImage, GenericImage};

impl VrellisCanvas {
    pub fn draw_image(&self) -> DynamicImage {
        let mut canvas = DynamicImage::new_rgb8(self.size_x, self.size_y).to_rgb();
        for item in &self.data {
            for (x, y, c) in item.data.image().to_rgb().enumerate_pixels() {
                unsafe { canvas.unsafe_put_pixel(x + item.x1 * self.grid, y + item.y1 * self.grid, *c) }
            }
        }
        return DynamicImage::ImageRgb8(canvas);
    }
}

impl VrellisCanvas {
    pub fn draw_canvas(&self) {}
}
