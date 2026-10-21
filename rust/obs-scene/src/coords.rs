//! Safe core of `libobs/obs-scene-coords.c.h`.
//!
//! The scene still reads the canvas and still decides the nudge and
//! keep-scale flags. This is only the math.

use obs_graphics::vec2::Vec2;

/// `obs_scene_coords_nudge`: true unless the item is a group or its parent is.
#[must_use]
pub fn coords_nudge(is_group: bool, parent_is_group: bool) -> bool {
    let _ = (is_group, parent_is_group);
    todo!("obs_scene_coords_nudge")
}

/// `obs_scene_coords_keep_scale`: true for a group or a nested scene.
#[must_use]
pub fn coords_keep_scale(is_group: bool, is_scene: bool) -> bool {
    let _ = (is_group, is_scene);
    todo!("obs_scene_coords_keep_scale")
}

/// `obs_scene_nudge_abs`: `floor(v * 2 + 0.5) / 2` on each axis.
#[must_use]
pub fn nudge_abs(v: Vec2) -> Vec2 {
    let _ = v;
    todo!("obs_scene_nudge_abs")
}

/// `obs_scene_pos_to_absolute`. Height is `[-1, 1]`, centered.
#[must_use]
pub fn pos_to_absolute(v: Vec2, canvas_x: f32, canvas_y: f32, nudge: bool) -> Vec2 {
    let _ = (v, canvas_x, canvas_y, nudge);
    todo!("obs_scene_pos_to_absolute")
}

/// `obs_scene_pos_from_absolute`. Does not nudge.
#[must_use]
pub fn pos_from_absolute(v: Vec2, canvas_x: f32, canvas_y: f32) -> Vec2 {
    let _ = (v, canvas_x, canvas_y);
    todo!("obs_scene_pos_from_absolute")
}

/// `obs_scene_size_to_absolute`. Canvas height is 2 in relative size.
#[must_use]
pub fn size_to_absolute(v: Vec2, canvas_y: f32, nudge: bool) -> Vec2 {
    let _ = (v, canvas_y, nudge);
    todo!("obs_scene_size_to_absolute")
}

/// `obs_scene_size_from_absolute`. Does not nudge.
#[must_use]
pub fn size_from_absolute(v: Vec2, canvas_y: f32) -> Vec2 {
    let _ = (v, canvas_y);
    todo!("obs_scene_size_from_absolute")
}

/// `obs_scene_item_canvas_scale`: `scale * (canvas_y / scale_ref_y)`.
#[must_use]
pub fn item_canvas_scale(scale: Vec2, canvas_y: f32, scale_ref_y: f32) -> Vec2 {
    let _ = (scale, canvas_y, scale_ref_y);
    todo!("obs_scene_item_canvas_scale")
}

/// `obs_scene_item_relative_scale`: `v * (scale_ref_y / canvas_y)`.
#[must_use]
pub fn item_relative_scale(v: Vec2, canvas_y: f32, scale_ref_y: f32) -> Vec2 {
    let _ = (v, canvas_y, scale_ref_y);
    todo!("obs_scene_item_relative_scale")
}
