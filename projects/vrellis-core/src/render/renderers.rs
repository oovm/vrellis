use crate::{MosaicCraftCanvasItem, MosaicCraftThemeItem, Result, Vrellis, VrellisCanvas};
use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgb};
use itertools::Itertools;
use std::{io::Cursor, path::Path, rc::Rc};

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
        let mut theme = self.theme.images.iter().map(|e| Rc::new(e.resized_item(self.grid_size))).collect_vec();
        if let Some(c) = self.background {
            let bg = ImageBuffer::from_pixel(self.grid_size, self.grid_size, c);
            theme.push(Rc::new(MosaicCraftThemeItem::new(c, DynamicImage::ImageRgb8(bg))))
        }
        let w = (img.width() as f32 * self.magnify).ceil() as u32 / self.grid_size;
        let h = (img.height() as f32 * self.magnify).ceil() as u32 / self.grid_size;
        let mut data = Vec::with_capacity(w as usize * h as usize);
        let resized = img.thumbnail_exact(w, h).to_rgb();
        for (x, y, c) in resized.enumerate_pixels() {
            let block = MosaicCraftCanvasItem { x1: x, y1: y, data: self.find_nearest_img(&theme, c) };
            data.push(block)
        }
        return VrellisCanvas { data, size_x: w * self.grid_size, size_y: h * self.grid_size, grid: self.grid_size };
    }
    fn find_nearest_img(&self, theme: &[Rc<MosaicCraftThemeItem>], color: &Rgb<u8>) -> Rc<MosaicCraftThemeItem> {
        unsafe {
            let min = theme.iter().min_by_key(|&rhs| self.color_metrics.distance(rgb_to_f32(*color), rhs.color) as u32);
            Rc::clone(min.unwrap())
        }
    }
}

pub unsafe fn rgb_to_f32(c: Rgb<u8>) -> (f32, f32, f32) {
    let r = *c.0.get_unchecked(0) as f32;
    let g = *c.0.get_unchecked(1) as f32;
    let b = *c.0.get_unchecked(2) as f32;
    (r, g, b)
}
