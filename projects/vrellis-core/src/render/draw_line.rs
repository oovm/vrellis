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
