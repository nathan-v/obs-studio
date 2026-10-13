//! The structs of `libobs/media-io/video-io.h` at the C boundary, and its
//! `static inline` helpers for enum values as they cross it.
//!
//! The helpers are compiled into every C caller, so nothing is exported or
//! swapped here. The functions below are for Rust shims that receive the
//! enums as `c_int`s: they convert, call the safe core in
//! [`crate::video_io`], and give a value outside its enum the result the
//! C gives it. A format outside `enum video_format` is not YUV and is named
//! `"None"`, like `VIDEO_FORMAT_NONE`; a colorspace outside
//! `enum video_colorspace` is named `"Unknown"`; a range outside
//! `enum video_range_type` is not `VIDEO_RANGE_DEFAULT`, so
//! `resolve_video_range` returns it unchanged and it is named `"Partial"`.

use core::ffi::{c_char, c_int, c_void};

use super::video_frame::video_frame;
use crate::video_io::{
    self, MAX_AV_PLANES, VideoFormat, video_colorspace_from_c, video_format_from_c,
    video_range_type_from_c, video_range_type_to_c,
};

/// Mirrors `struct video_data`.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct video_data {
    pub data: [*mut u8; MAX_AV_PLANES],
    pub linesize: [u32; MAX_AV_PLANES],
    pub timestamp: u64,
}

/// Mirrors `struct video_output_info`. The enums are `c_int`s.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct video_output_info {
    pub name: *const c_char,

    pub format: c_int,
    pub fps_num: u32,
    pub fps_den: u32,
    pub width: u32,
    pub height: u32,
    pub cache_size: usize,

    pub colorspace: c_int,
    pub range: c_int,
}

/// Mirrors `struct video_scale_info`. The enums are `c_int`s.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct video_scale_info {
    pub format: c_int,
    pub width: u32,
    pub height: u32,
    pub range: c_int,
    pub colorspace: c_int,
}

/// `format_is_yuv` for a C `enum video_format` value.
pub fn format_is_yuv(format: c_int) -> bool {
    video_format_from_c(format).is_some_and(video_io::format_is_yuv)
}

/// `get_video_format_name` for a C `enum video_format` value.
pub fn get_video_format_name(format: c_int) -> &'static str {
    video_io::video_format_name(video_format_from_c(format).unwrap_or(VideoFormat::None))
}

/// `get_video_colorspace_name` for a C `enum video_colorspace` value.
pub fn get_video_colorspace_name(cs: c_int) -> &'static str {
    video_colorspace_from_c(cs).map_or("Unknown", video_io::video_colorspace_name)
}

/// `resolve_video_range` for C `enum video_format` and
/// `enum video_range_type` values.
pub fn resolve_video_range(format: c_int, range: c_int) -> c_int {
    let format = video_format_from_c(format).unwrap_or(VideoFormat::None);
    video_range_type_from_c(range).map_or(range, |range| {
        video_range_type_to_c(video_io::resolve_video_range(format, range))
    })
}

/// `get_video_range_name` for C `enum video_format` and
/// `enum video_range_type` values.
pub fn get_video_range_name(format: c_int, range: c_int) -> &'static str {
    let format = video_format_from_c(format).unwrap_or(VideoFormat::None);
    video_range_type_from_c(range)
        .map_or("Partial", |range| video_io::video_range_name(format, range))
}

/// `VIDEO_OUTPUT_SUCCESS`.
pub const VIDEO_OUTPUT_SUCCESS: c_int = 0;
/// `VIDEO_OUTPUT_INVALIDPARAM`.
pub const VIDEO_OUTPUT_INVALIDPARAM: c_int = -1;
/// `VIDEO_OUTPUT_FAIL`.
pub const VIDEO_OUTPUT_FAIL: c_int = -2;

/// `void (*)(void *param, struct video_data *frame)`.
#[allow(non_camel_case_types)]
pub type video_output_callback_t =
    Option<unsafe extern "C" fn(param: *mut c_void, frame: *mut video_data)>;

// Stubs. The Tier 1 tests call these and fail here until the port lands.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_open(
    _video: *mut *mut c_void,
    _info: *mut video_output_info,
) -> c_int {
    todo!("video_output_open")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_close(_video: *mut c_void) {
    todo!("video_output_close")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_connect(
    _video: *mut c_void,
    _conversion: *const video_scale_info,
    _callback: video_output_callback_t,
    _param: *mut c_void,
) -> bool {
    todo!("video_output_connect")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_connect2(
    _video: *mut c_void,
    _conversion: *const video_scale_info,
    _frame_rate_divisor: u32,
    _callback: video_output_callback_t,
    _param: *mut c_void,
) -> bool {
    todo!("video_output_connect2")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_disconnect(
    _video: *mut c_void,
    _callback: video_output_callback_t,
    _param: *mut c_void,
) {
    todo!("video_output_disconnect")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_disconnect2(
    _video: *mut c_void,
    _callback: video_output_callback_t,
    _param: *mut c_void,
) -> bool {
    todo!("video_output_disconnect2")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_active(_video: *const c_void) -> bool {
    todo!("video_output_active")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_get_info(_video: *const c_void) -> *const video_output_info {
    todo!("video_output_get_info")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_lock_frame(
    _video: *mut c_void,
    _frame: *mut video_frame,
    _count: c_int,
    _timestamp: u64,
) -> bool {
    todo!("video_output_lock_frame")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_unlock_frame(_video: *mut c_void) {
    todo!("video_output_unlock_frame")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_get_frame_time(_video: *const c_void) -> u64 {
    todo!("video_output_get_frame_time")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_stop(_video: *mut c_void) {
    todo!("video_output_stop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_stopped(_video: *mut c_void) -> bool {
    todo!("video_output_stopped")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_get_format(_video: *const c_void) -> c_int {
    todo!("video_output_get_format")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_get_width(_video: *const c_void) -> u32 {
    todo!("video_output_get_width")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_get_height(_video: *const c_void) -> u32 {
    todo!("video_output_get_height")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_get_frame_rate(_video: *const c_void) -> f64 {
    todo!("video_output_get_frame_rate")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_get_skipped_frames(_video: *const c_void) -> u32 {
    todo!("video_output_get_skipped_frames")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_get_total_frames(_video: *const c_void) -> u32 {
    todo!("video_output_get_total_frames")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_inc_texture_encoders(_video: *mut c_void) {
    todo!("video_output_inc_texture_encoders")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_dec_texture_encoders(_video: *mut c_void) {
    todo!("video_output_dec_texture_encoders")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_inc_texture_frames(_video: *mut c_void) {
    todo!("video_output_inc_texture_frames")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_inc_texture_skipped_frames(_video: *mut c_void) {
    todo!("video_output_inc_texture_skipped_frames")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_create_with_frame_rate_divisor(
    _video: *mut c_void,
    _divisor: u32,
) -> *mut c_void {
    todo!("video_output_create_with_frame_rate_divisor")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn video_output_free_frame_rate_divisor(_video: *mut c_void) {
    todo!("video_output_free_frame_rate_divisor")
}
