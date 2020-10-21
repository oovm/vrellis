mod renderers;

use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

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
    pub fn sample(&self, num: u32, width: u32, height: u32) -> Vec<VrellisPoint> {
        assert!(num > 9, "too less samples!");
        let mut out = Vec::with_capacity(num as usize);
        match self {
            VrellisShape::Circle => {
                for n in 0..num {
                    let x = 1.0 + (n as f32).cos();
                    let y = 1.0 + (n as f32).sin();
                    out.push(VrellisPoint { n, x: (x * width as f32).round() as u32, y: (y * height as f32).round() as u32 })
                }
            }
            VrellisShape::Triangle => {
                assert_eq!(num % 3, 0, "Must be a multiple of 3");
                unimplemented!()
            }
            VrellisShape::Square => {
                let poly = Self::Polygon { corners: vec![(0, 0), (0, w), (w, h), (h, 0)] };
                poly.sample(num, width, height)
            }
            VrellisShape::Polygon { corners } => {
                // FIXME: better way to get shift pair
                let mut shifted = VecDeque::from(corners.clone());
                let head = shifted.pop_front().unwrap();
                shifted.push_back(head);

                let mut circumference = 0.0;
                let mut temp_line = vec![];
                for (a,b) in corners.iter().zip(shifted.iter()) {
                    let c = (a.0 - b.0).powf(2) + (a.1 - b.1).powf(2);
                    let z = (c as f32).sqrt();
                    let p1 = circumference;
                    circumference +=  z;
                    let p2 = circumference;
                    temp_line.push(VrellisLine {
                        p1,
                        p2,
                        x1: a.0,
                        y1: a.1,
                        x2: b.0,
                        y2: b.1,
                        z
                    })

                }


                corners


            },
            VrellisShape::Parabola => unimplemented!(),
            VrellisShape::Custom { points } => points.clone(),
        }
        return out;
    }
}

struct VrellisLine {
    p1: f32,
    p2: f32,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    z: f32,
}

impl VrellisLine {
    fn cos(&self) -> f32 {
        (self.x1 - self.x2).abs() as f32 / self.z
    }
    fn sin(&self) -> f32 {
        (self.y1 - self.y2).abs() as f32 / self.z
    }
    fn rescale_p(&self, p: f32) -> f32 {
        (p - self.p1) / (self.p2 - self.p1)
    }
    fn percent_x(&self, p: f32) -> f32 {
        self.x1 + self.rescale_p(p) * self.cos()
    }
    fn percent_y(&self, p: f32) -> f32 {
        self.y1 + self.rescale_p(p) * self.sin()
    }
    fn line_percent_position(&self, p: f32) -> (u32, u32) {
        (self.percent_x(p).round() as u32, self.percent_y(p).round() as u32)
    }
}
