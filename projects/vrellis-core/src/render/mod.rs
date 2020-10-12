mod renderers;

use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum VrellisShape {
    Circle = 0,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct VrellisPoint {
    n: u32,
    x:u32,
    y:u32,
}

impl Default for VrellisShape {
    fn default() -> Self {
        Self::Circle
    }
}

impl Default for VrellisPoint {
    fn default() -> Self {
        Self {
            n: 0,
            x: 0,
            y: 0
        }
    }
}


impl VrellisShape {
    pub fn sample(&self, num: u32, scale: f32) -> Vec<VrellisPoint> {
        let mut out = Vec::with_capacity(num as usize);
        match self {
            VrellisShape::Circle => {
                for n in 0..num {
                    let x = (scale/2.0) * (n as f32).cos();
                    let y = (scale/2.0) * (n as f32).sin();
                    out.push(VrellisPoint {
                        n,
                        x: x as u32,
                        y: y as u32
                    })
                }



            }
        }
        return out
    }
}

