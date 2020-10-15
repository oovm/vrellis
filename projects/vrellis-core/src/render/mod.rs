mod renderers;

use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VrellisShape {
    Circle { scale: f32 },
    Triangle { scale: f32, rotate: f32 },
    Square { scale: f32, rotate: f32 },
    Polygon { edges: Vec<(u32, u32)> },
}



#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct VrellisPoint {
    n: u32,
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum VrellisColorMode {
    Grayscale = 0,
    Colorful = 1,
    LayerMask = 2,
}

impl Default for VrellisShape {
    fn default() -> Self {
        Self::Circle
    }
}

impl Default for VrellisPoint {
    fn default() -> Self {
        Self { n: 0, x: 0, y: 0 }
    }
}

impl Default for VrellisColorMode {
    fn default() -> Self {
        Self::Grayscale
    }
}

impl VrellisShape {
    pub fn sample(&self, num: u32) -> Vec<VrellisPoint> {
        assert!(num > 9, "too less samples!");
        let mut out = Vec::with_capacity(num as usize);
        match self {
            VrellisShape::Circle { scale } => {
                for n in 0..num {
                    let x = (scale / 2.0) * (n as f32).cos();
                    let y = (scale / 2.0) * (n as f32).sin();
                    out.push(VrellisPoint { n, x: x as u32, y: y as u32 })
                }
            }
            VrellisShape::Triangle { .. } => unimplemented!(),
            VrellisShape::Square { .. } => unimplemented!(),
            VrellisShape::Polygon { .. } => unimplemented!(),
        }
        return out;
    }
}
