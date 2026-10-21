//! Placement, crop, and flip adjustments for scene bounds.
use crate::bounds::BoundsType;
use obs_graphics::vec2::Vec2;

pub const LEFT: u32 = 1;
pub const RIGHT: u32 = 2;
pub const TOP: u32 = 4;
pub const BOTTOM: u32 = 8;
pub const CENTER: u32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Crop {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub origin: Vec2,
    pub scale: Vec2,
    pub cx: u32,
    pub cy: u32,
    pub crop: Crop,
}

pub fn add_alignment(_v: Vec2, _align: u32, _cx: i32, _cy: i32) -> Vec2 {
    todo!()
}
pub fn bounds_place(
    _kind: BoundsType,
    _bounds: Vec2,
    _item: Placement,
    _align: u32,
    _crop_to_bounds: bool,
) -> Placement {
    todo!()
}
