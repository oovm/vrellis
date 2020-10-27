mod solver;
use serde::{Deserialize, Serialize};
pub use solver::VrellisAlgorithm;
use std::{collections::VecDeque, f32::consts::PI};
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct VrellisPoint {
    pub n: u32,
    pub x: u32,
    pub y: u32,
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
        assert!(num > 7, "too less samples!");
        let mut out = Vec::with_capacity(num as usize);
        match self {
            VrellisShape::Circle => {
                for n in 0..num {
                    let x = 1.0 + (-2.0 * n as f32 * PI / num as f32).cos();
                    let y = 1.0 + (-2.0 * n as f32 * PI / num as f32).sin();
                    out.push(VrellisPoint {
                        n,
                        x: (x * width as f32 / 2.0).round() as u32,
                        y: (y * height as f32 / 2.0).round() as u32,
                    })
                }
            }
            VrellisShape::Triangle => {
                assert_eq!(num % 3, 0, "Must be a multiple of 3");
                unimplemented!()
            }
            VrellisShape::Square => {
                let poly = Self::Polygon { corners: vec![(0, 0), (width, 0), (width, height), (0, height)] };
                return poly.sample(num, 1, 1);
            }
            VrellisShape::Polygon { corners } => {
                // FIXME: better way to get shift pair
                let mut shifted = VecDeque::from(corners.clone());
                let head = shifted.pop_front().unwrap();
                shifted.push_back(head);
                // build edges
                let mut circumference = 0.0;
                let mut temp_line = vec![];
                for (a, b) in corners.iter().zip(shifted.iter()) {
                    let p1 = circumference;
                    circumference += ((a.0 as f32 - b.0 as f32).powf(2.0) + (a.1 as f32 - b.1 as f32).powf(2.0)).sqrt();
                    let p2 = circumference;
                    temp_line.push(VrellisLine { p1, p2, x1: a.0, y1: a.1, x2: b.0, y2: b.1 });
                }
                temp_line.iter_mut().for_each(|e| e.resize(circumference));
                // find points
                let mut edges = temp_line.into_iter();
                let mut this_edge = edges.next().unwrap();
                for n in 0..num {
                    let percent = n as f32 / num as f32;
                    while percent > this_edge.p2 {
                        match edges.next() {
                            Some(s) => {
                                this_edge = s;
                            }
                            None => {
                                return out;
                            }
                        }
                    }
                    let (x, y) = this_edge.get_percent_position(percent);
                    out.push(VrellisPoint { n, x: x.round() as u32, y: y.round() as u32 })
                }
            }
            VrellisShape::Parabola => unimplemented!(),
            VrellisShape::Custom { points } => return points.clone(),
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
}

impl VrellisLine {
    fn resize(&mut self, c: f32) {
        self.p1 /= c;
        self.p2 /= c;
    }
    fn rescale_p(&self, p: f32) -> f32 {
        (p - self.p1) / (self.p2 - self.p1)
    }
    fn percent_x(&self, p: f32) -> f32 {
        self.x1 as f32 + self.rescale_p(p) * (self.x2 as f32 - self.x1 as f32)
    }
    fn percent_y(&self, p: f32) -> f32 {
        self.y1 as f32 + self.rescale_p(p) * (self.y2 as f32 - self.y1 as f32)
    }
    fn get_percent_position(&self, p: f32) -> (f32, f32) {
        assert!(self.p1 <= p && p <= self.p2);
        (self.percent_x(p), self.percent_y(p))
    }
}
