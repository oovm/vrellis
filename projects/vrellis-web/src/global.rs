use serde::{Deserialize, Serialize};
use vrellis_core::{Vrellis, VrellisAlgorithm, VrellisColorMode, VrellisShape};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GlobalSettings {
    pub steps: u32,
    pub points: u32,
    pub convex_shape: VrellisShape,
    pub color_mode: VrellisColorMode,
    pub anti_aliased: bool,
    pub inverted_color: bool,
    pub min_distance: u32,
    pub line_width: f32,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            steps: 1000,
            convex_shape: Default::default(),
            points: 200,
            color_mode: Default::default(),
            anti_aliased: true,
            inverted_color: false,
            min_distance: 0,
            line_width: 0.5,
        }
    }
}

impl GlobalSettings {
    pub fn build(&self) -> Vrellis {
        let algorithm = match self.anti_aliased {
            true => VrellisAlgorithm::AntiAliased,
            false => VrellisAlgorithm::ThinLine,
        };
        Vrellis {
            convex_shape: self.convex_shape.clone(),
            points: self.points,
            color_mode: self.color_mode,
            algorithm,
            inverted_color: false,
            min_distance: self.min_distance,
            line_width: self.line_width,
        }
    }
}
