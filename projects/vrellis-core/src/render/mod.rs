mod renderers;

use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ColorAverage {
    RGBSpace = 0,
}

#[derive(Debug, Copy, Clone)]
pub enum ColorMetrics {
    /// L1 distance
    Manhattan = 0,
    /// L2 distance
    Euclid = 1,
}

impl Default for ColorAverage {
    fn default() -> Self {
        Self::RGBSpace
    }
}

impl Default for ColorMetrics {
    fn default() -> Self {
        Self::Manhattan
    }
}

impl ColorAverage {
    pub fn mean(&self, img: &DynamicImage) -> (f32, f32, f32) {
        match self {
            ColorAverage::RGBSpace => {
                let all = img.width() as f32 * img.height() as f32;
                let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
                for c in img.to_rgb().pixels() {
                    unsafe {
                        r += *c.0.get_unchecked(0) as f32;
                        g += *c.0.get_unchecked(1) as f32;
                        b += *c.0.get_unchecked(2) as f32;
                    }
                }
                (r / all, g / all, b / all)
            }
        }
    }
}

impl ColorMetrics {
    pub fn distance(&self, lhs: (f32, f32, f32), rhs: (f32, f32, f32)) -> f32 {
        match self {
            ColorMetrics::Manhattan => {
                let dx = (lhs.0 - rhs.0).abs();
                let dy = (lhs.1 - rhs.1).abs();
                let dz = (lhs.2 - rhs.2).abs();
                dx + dy + dz
            }
            ColorMetrics::Euclid => {
                let dx = (lhs.0 - rhs.0).powf(2.0);
                let dy = (lhs.1 - rhs.1).powf(2.0);
                let dz = (lhs.2 - rhs.2).powf(2.0);
                (dx + dy + dz).sqrt()
            }
        }
    }
}
