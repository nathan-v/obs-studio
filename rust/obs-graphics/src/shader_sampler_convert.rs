//! Safe sampler-state conversion. The adapter retains the native C runtime's
//! numeric parsing (including long width, locale and errno) behind `number`.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SamplerInfo {
    pub filter: i32,
    pub address_u: i32,
    pub address_v: i32,
    pub address_w: i32,
    pub max_anisotropy: i32,
    pub border_color: u32,
}

/// Read-only sampler view. Numeric conversion is supplied by the host adapter;
/// `offset` skips the border-color prefix and `radix` is 10 or 16.
pub trait Sampler {
    fn state_count(&self) -> usize;
    fn state(&self, index: usize) -> &[u8];
    fn value(&self, index: usize) -> &[u8];
    fn number(&self, index: usize, offset: usize, radix: u32) -> i128;
}

pub fn convert(_sampler: &impl Sampler) -> SamplerInfo {
    todo!("shader sampler conversion")
}
