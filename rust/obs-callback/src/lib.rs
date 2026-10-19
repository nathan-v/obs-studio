//! Rust port of `libobs/callback/calldata.c.h`.
//!
//! [`calldata`] is the safe core.

#![deny(unsafe_code)]

pub mod calldata;

pub use calldata::{Calldata, FIXED_OVERFLOW_MESSAGE, StringGet};
