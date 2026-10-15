//! Layout of the vec and frame-rate values the obs-data shim passes through.
//! The C function shims land with the port.

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default)]
pub struct vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C, align(16))]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default)]
pub struct vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C, align(16))]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default)]
pub struct vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C, align(16))]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default)]
pub struct quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default)]
pub struct media_frames_per_second {
    pub numerator: u32,
    pub denominator: u32,
}
