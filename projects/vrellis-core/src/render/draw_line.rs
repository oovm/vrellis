use image::{GrayAlphaImage, GrayImage, Luma, LumaA};
use imageproc::drawing::{draw_antialiased_line_segment_mut, draw_line_segment_mut, BresenhamLinePixelIter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn line_score(&self, img: &GrayImage, p1: (u32, u32), p2: (u32, u32)) -> f32 {
        let line = BresenhamLinePixelIter::<Luma<u8>>::new(img, (p1.0 as f32, p1.1 as f32), (p2.0 as f32, p2.1 as f32));
        let mut sum = 0.0;
        for p in line {
            unsafe { sum += p.0.get_unchecked(0) as f32 }
        }
        return sum;
    }
    pub fn draw_line(&self, img: &mut GrayImage, p1: (u32, u32), p2: (u32, u32), reversed: bool) {
        let pixel = match reversed {
            true => Luma([0]),
            false => Luma([255]),
        };
        match self {
            VrellisAlgorithm::NonRendered => (),
            VrellisAlgorithm::ThinLine => {
                draw_line_segment_mut(img, (p1.0 as f32, p1.1 as f32), (p2.0 as f32, p2.1 as f32), pixel)
            }
            VrellisAlgorithm::AntiAliased => draw_antialiased_line_segment_mut(
                img,
                (p1.0 as i32, p1.1 as i32),
                (p2.0 as i32, p2.1 as i32),
                pixel,
                |a, b, c| {
                    let mix = c * (b as f32 - a as f32);
                    Luma([a + mix.round() as u8])
                },
            ),
        }
    }
}
