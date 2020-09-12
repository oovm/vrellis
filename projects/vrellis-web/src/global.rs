use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GlobalSettings {
    pub scene: Scene,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Scene {
    AsciiArt,
    BrailleArt,
    EmojiArt,
}

impl Debug for Scene {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Scene::AsciiArt => f.debug_tuple("AsciiArt").finish(),
            Scene::BrailleArt => f.debug_tuple("BrailleArt").finish(),
            Scene::EmojiArt => f.debug_tuple("EmojiArt").finish(),
        }
    }
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self { scene: Scene::AsciiArt }
    }
}

pub fn format_image_size(img: &Option<DynamicImage>) -> String {
    match img {
        Some(i) => format!("{}×{}", i.width(), i.height()),
        None => String::from("0"),
    }
}
