//! Safe core of `obs_bounds_fit_scale`.

use obs_graphics::vec2::Vec2;

/// `enum obs_bounds_type` in `libobs/obs.h`, same numeric order.
///
/// [`BoundsType::Other`] is a value outside that enum. C matches no branch
/// and leaves the scale unchanged, the same as [`BoundsType::None`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BoundsType {
    None = 0,
    Stretch = 1,
    ScaleInner = 2,
    ScaleOuter = 3,
    ScaleToWidth = 4,
    ScaleToHeight = 5,
    MaxOnly = 6,
    Other = -1,
}

impl BoundsType {
    /// The C enumerator, or [`BoundsType::Other`] for any other `int`.
    #[must_use]
    pub fn from_c(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Stretch,
            2 => Self::ScaleInner,
            3 => Self::ScaleOuter,
            4 => Self::ScaleToWidth,
            5 => Self::ScaleToHeight,
            6 => Self::MaxOnly,
            _ => Self::Other,
        }
    }
}

/// Fit `scale` for a `cx` by `cy` source drawn into `bounds`.
///
/// A zero size on the axis a fit would divide by leaves `scale` unchanged,
/// so an empty source stays finite.
#[must_use]
pub fn bounds_fit_scale(kind: BoundsType, bounds: Vec2, cx: u32, cy: u32, scale: Vec2) -> Vec2 {
    let _ = (kind, bounds, cx, cy, scale);
    todo!("obs_bounds_fit_scale")
}
