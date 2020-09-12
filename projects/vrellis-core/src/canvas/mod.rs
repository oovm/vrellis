mod draw;
mod save;

use crate::MosaicCraftThemeItem;

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct MosaicCraftCanvas {
    pub data: Vec<MosaicCraftCanvasItem>,
    pub size_x: u32,
    pub size_y: u32,
    pub grid: u32,
}

#[derive(Debug, Clone)]
pub struct MosaicCraftCanvasItem {
    pub x1: u32,
    pub y1: u32,
    pub data: Rc<MosaicCraftThemeItem>,
}
