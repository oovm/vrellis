mod renderers;

use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VrellisShape {
    Circle,
    Triangle,
    Square,
    Parabola,
    /// Note that it must be a convex hull
    Polygon {
        corners: Vec<(u32, u32)>,
    },
    /// Note that it must be a convex curve
    Custom {
        points: Vec<VrellisPoint>,
    },
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
    pub fn sample(&self, num: u32, width: f32, height: f32) -> Vec<VrellisPoint> {
        assert!(num > 9, "too less samples!");
        let mut out = Vec::with_capacity(num as usize);
        match self {
            VrellisShape::Circle => {
                for n in 0..num {
                    let x = 1.0 + (n as f32).cos();
                    let y = 1.0 + (n as f32).sin();
                    out.push(VrellisPoint { n, x: (x * width).round() as u32, y: (y * height).round() as u32 })
                }
            }
            VrellisShape::Triangle => {
                assert_eq!(num % 4, 0, "Must be a multiple of 3");
                unimplemented!()
            }
            VrellisShape::Square => {
                assert_eq!(num % 4, 0, "Must be a multiple of 4");
                for n in 0..(num / 4) {
                    let w = width as u32;
                    out.push(VrellisPoint { n: 0 * num / 4 + n, x: x as u32, y: 0 });
                    out.push(VrellisPoint { n: 1 * num / 4 + n, x: w, y: y as u32 });
                    out.push(VrellisPoint { n: 2 * num / 4 + n, x: x as u32, y: y as u32 });
                    out.push(VrellisPoint { n: 3 * num / 4 + n, x: x as u32, y: y as u32 })
                }
            }
            VrellisShape::Polygon { .. } => unimplemented!(),
            VrellisShape::Parabola => unimplemented!(),
            VrellisShape::Custom { points } => points.clone(),
        }
        return out;
    }
}

fn line_percent_position(percent:f32, p1:(f32,u32, u32), p2:(f32,u32, u32))-> (u32,u32) {
    assert!(p1.0<percent && percent<p2.0);

    x = x0+cos t
y =y0+sin t

    unimplemented!()
}


struct VrellisLine  {
    p1:f32,
    p2:f32,
    x1:u32,
    y1:u32,
    x2:u32,
   y2:u32,
    z: f32
}

impl  VrellisLine {
    fn cos(&self)-> f32 {
        (self.x1- self.x2).abs() as f32 / self.z
    }
    fn sin(&self)-> f32 {
        (self.y1- self.y2).abs() as f32 / self.z
    }
    fn rescale_p(&self, p: f32) ->f32{

    }
    fn percent_x(&self, p: f32) ->f32 {
        self.x1 + self.rescale_p()* self.cos()
    }
    fn percent_y(&self, p: f32) ->f32{
        self.y1 + self.rescale_p()* self.sin()
    }
    fn line_percent_position(&self, p: f32) ->(u32,u32) {

    }
}