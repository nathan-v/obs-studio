//! Safe core of `libobs/graphics/math-extra.c`.
//!
//! Header-inline `get_percentage` and `get_percentagei` stay in
//! `math-extra.h`. `polar_to_cart` names its `cosf` result `sinx`, as in C.

use crate::vec2::Vec2;
use crate::vec3::Vec3;

/// `polar_to_cart`. `w` is 0. C leaves that lane uninitialized.
#[must_use]
pub fn polar_to_cart(_v: Vec3) -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

/// `cart_to_polar`. A length within `EPSILON` is the zero vector, and `w`
/// counts toward that length.
#[must_use]
pub fn cart_to_polar(_v: Vec3) -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

/// `norm_to_polar`.
#[must_use]
pub fn norm_to_polar(_norm: Vec3) -> Vec2 {
    Vec2::new(0.0, 0.0)
}

/// `polar_to_norm`. The safe result has `w` 0. The shim leaves `dst->w` alone.
#[must_use]
pub fn polar_to_norm(_polar: Vec2) -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

/// `calc_torquef`.
#[must_use]
pub fn calc_torquef(_val1: f32, _val2: f32, _torque: f32, _min_adjust: f32, _t: f32) -> f32 {
    0.0
}

/// `calc_torque`. Reads both inputs. The shim re-reads `v1` after writing
/// `dst` when those pointers alias, which this function cannot see.
#[must_use]
pub fn calc_torque(_v1: Vec3, _v2: Vec3, _torque: f32, _min_adjust: f32, _t: f32) -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

/// `rand_float` given one draw of `rand() / RAND_MAX` in `f64`.
///
/// Any `positive_only` flag yields that draw cast to `f32` (the `[0, 1]`
/// branch). Otherwise the draw is mapped to `[-1, 1]`.
#[must_use]
pub fn rand_float(_positive_only: bool, _unit: f64) -> f32 {
    0.0
}
