mod canvas;
mod errors;
mod render;

pub use crate::{
    canvas::VrellisCanvas,
    errors::{Result, VrellisError},
    render::{VrellisAlgorithm, VrellisColorMode, VrellisPoint, VrellisShape},
};
pub use image::{Luma, Rgb};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vrellis {
    /// The shape of the uniformly distributed point set on the convex hull
    pub convex_shape: VrellisShape,
    /// number of points
    pub points: u32,
    /// Color rendering mode, not all drawing modes are supported
    pub color_mode: VrellisColorMode,
    /// Algorithm to use when evaluating the next step
    pub algorithm: VrellisAlgorithm,
    /// `true`: Use multiply mode to paint on the black board
    /// `false`: Use filter mode to paint on the white board
    pub inverted_color: bool,
    /// If the ordinal difference of two points is less than or equal to this value
    /// Then there will be no connection between the two
    pub min_distance: u32,
    /// The basic width of the line when drawing
    pub line_width: f32,
    /* Whether to use highlight color to highlight the last step
     * pub highlight_last_step: Option<Rgb<u8>>, */
}

impl Default for Vrellis {
    fn default() -> Self {
        Self {
            convex_shape: Default::default(),
            color_mode: Default::default(),
            algorithm: Default::default(),
            inverted_color: false,
            points: 100,
            min_distance: 0,
            line_width: 1.0,
        }
    }
}
