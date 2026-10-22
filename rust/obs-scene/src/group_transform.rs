//! Safe core of `libobs/obs-scene-group-transform.c.h`.
//!
//! The scene still reads the item, still converts absolute and relative
//! coordinates, still inverts the draw transform when an item joins a
//! group, and still calls `update_item_transform`. This is only the matrix
//! math.

use obs_graphics::matrix4::Matrix4;
use obs_graphics::vec2::Vec2;

/// `obs_scene_transform_val`. Not implemented yet.
#[must_use]
pub fn transform_val(point: Vec2, transform: &Matrix4) -> Vec2 {
    let _ = (point, transform);
    todo!("obs_scene_transform_val")
}

/// Position, scale, and rotation after `obs_scene_group_transform`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GroupTransform {
    pub pos: Vec2,
    pub scale: Vec2,
    pub rot: f32,
}

/// `obs_scene_group_transform`. Not implemented yet.
#[must_use]
pub fn group_transform(
    transform: &Matrix4,
    pos: Vec2,
    scale: Vec2,
    rot: f32,
    rot_delta: f32,
) -> GroupTransform {
    let _ = (transform, pos, scale, rot, rot_delta);
    todo!("obs_scene_group_transform")
}
