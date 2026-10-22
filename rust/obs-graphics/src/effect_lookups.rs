//! Safe core for `libobs/graphics/effect-lookups.c.h`.
//!
//! Name walks are a first-match `strcmp` on the bytes before the NUL.
//! Param indexes stop at the count (`>=` is a miss). Annotation indexes
//! and pass indexes use `>`, so an index equal to the count is one past
//! the array. That return is characterized, not endorsed.

/// Where an annotation or pass index lands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LooseIndex {
    /// `index < len`.
    At(usize),
    /// `index == len`. C returns `array + len` and does not dereference it.
    OnePast,
    /// Null owner, or `index > len`.
    Miss,
}

/// `gs_effect_get_num_params` / `gs_param_get_num_annotations`.
pub fn count(present: bool, len: usize) -> usize {
    let _ = (present, len);
    todo!("gs_effect_get_num_params")
}

/// `gs_effect_get_param_by_idx`. `index >= len` is a miss.
pub fn param_index(present: bool, len: usize, index: usize) -> Option<usize> {
    let _ = (present, len, index);
    todo!("gs_effect_get_param_by_idx")
}

/// `gs_param_get_annotation_by_idx` / `gs_technique_get_pass_by_idx`.
pub fn loose_index(present: bool, len: usize, index: usize) -> LooseIndex {
    let _ = (present, len, index);
    todo!("gs_param_get_annotation_by_idx")
}

/// First exact byte match, or `None` when the owner is null or the name
/// is missing. `gs_effect_get_param_by_name`, `gs_param_get_annotation_by_name`,
/// `gs_effect_get_technique`, `gs_technique_get_pass_by_name`.
pub fn find_name(present: bool, names: &[&[u8]], name: &[u8]) -> Option<usize> {
    let _ = (present, names, name);
    todo!("name lookup")
}

/// `gs_effect_get_current_technique`, `gs_effect_get_viewproj_matrix`,
/// `gs_effect_get_world_matrix`. A null owner returns null. Otherwise the
/// stored pointer is returned as it is, including a pointer that is not
/// in the owner's array.
pub fn stored<T>(present: bool, value: *const T) -> *const T {
    let _ = (present, value);
    todo!("stored pointer")
}

/// Name pointer and type copied by `gs_effect_get_param_info`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParamInfo<'a> {
    pub name: &'a [u8],
    pub type_: i32,
}

/// `gs_effect_get_param_info`. `None` means the param was null and the
/// caller must leave `info` untouched.
pub fn param_info(param: Option<(&[u8], i32)>) -> Option<ParamInfo<'_>> {
    let _ = param;
    todo!("gs_effect_get_param_info")
}
