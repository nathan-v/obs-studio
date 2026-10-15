//! Rust port of `libobs/obs-data.c`.
//!
//! The safe core is [`data`]. The C ABI shims live in [`ffi`].

#![deny(unsafe_code)]

pub mod data;

#[allow(unsafe_code)]
pub mod ffi;

pub use data::{Array, Data, Fps, FpsRead, Item, NumType, Quat, Type, Vec2, Vec3, Vec4};
