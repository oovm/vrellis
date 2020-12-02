use crate::VrellisCanvas;
use image::{GrayImage, Luma, LumaA};
use imageproc::drawing::{draw_antialiased_line_segment_mut, draw_line_segment_mut, BresenhamLinePixelIter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum VrellisAlgorithm {
    /// Not actually render on the original image
    NonRendered,
    /// Render all the passed pixels to maximum values
    ThinLine,
    /// Render an anti-aliased line with a velocity of 64
    AntiAliased,
}

impl Default for VrellisAlgorithm {
    fn default() -> Self {
        Self::AntiAliased
    }
}

impl VrellisAlgorithm {
    pub fn line_score(&self, img: &GrayImage, x1: u32, x2: u32, y1: u32, y2: u32, reversed: bool) -> f32 {
        let line = BresenhamLinePixelIter::<Luma<u8>>::new(img, (x1 as f32, x2 as f32), (y1 as f32, y2 as f32));
        let mut sum = 0.0;
        for p in line {
            let s = unsafe { *p.0.get_unchecked(0) };
            sum += match reversed {
                true => s as f32,
                false => (255 - s) as f32,
            }
        }
        return sum;
    }
    pub fn draw_line(&self, img: &mut GrayImage, x1: u32, x2: u32, y1: u32, y2: u32, reversed: bool) {
        let pixel = match reversed {
            true => Luma([0]),
            false => Luma([255]),
        };
        match self {
            VrellisAlgorithm::NonRendered => (),
            VrellisAlgorithm::ThinLine => draw_line_segment_mut(img, (x1 as f32, x2 as f32), (y1 as f32, y2 as f32), pixel),
            VrellisAlgorithm::AntiAliased => draw_antialiased_line_segment_mut(
                img,
                (x1 as i32, x2 as i32),
                (y1 as i32, y2 as i32),
                pixel,
                |a, b, c| unsafe {
                    let a = *a.0.get_unchecked(0) as f32;
                    let b = *b.0.get_unchecked(0) as f32;
                    let mix = a + c * (b - a);
                    Luma([mix.round() as u8])
                },
            ),
        }
    }
}

#[allow(dead_code)]
impl VrellisCanvas {
    pub(in crate::render) fn draw_canvas_line(&mut self, x1: u32, x2: u32, y1: u32, y2: u32, reversed: bool) {
        let pixel = match reversed {
            true => LumaA([255, 255]),
            false => LumaA([0, 255]),
        };
        draw_line_segment_mut(&mut self.current_image, (x1 as f32, x2 as f32), (y1 as f32, y2 as f32), pixel)
    }
}
