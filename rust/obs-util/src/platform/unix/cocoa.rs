//! macOS half of `libobs/util/platform-cocoa.m`.
//!
//! Empty stubs. The Tier 1 tests fail here on purpose.

#![allow(unsafe_code)]

use std::ffi::c_void;

use crate::dstr::Dstr;

/// `os_get_path_internal`: null or empty `name` is `base`, otherwise `base/name`.
pub fn format_path(_base: &[u8], _name: Option<&[u8]>) -> Vec<u8> {
    todo!("platform-cocoa")
}

/// `os_get_path_ptr_internal`: always appends `/`, then `name` when it is non-empty.
pub fn format_path_ptr(_base: &[u8], _name: Option<&[u8]>) -> Vec<u8> {
    todo!("platform-cocoa")
}

/// `(double)sys_delta * 100.0 / cpu_delta / core_count`.
pub fn cpu_percent(_sys_delta: i64, _cpu_delta: i64, _core_count: i32) -> f64 {
    todo!("platform-cocoa")
}

/// `os_inhibit_sleep_create` checks `if (!reason)`.
pub fn inhibit_uses_null_cstring(_reason_is_null: bool) -> bool {
    todo!("platform-cocoa")
}

/// `os_get_free_disk_space` casts the signed free-space value.
pub fn free_disk_cast(_free: i64) -> u64 {
    todo!("platform-cocoa")
}

/// `os_get_config_path`.
pub fn config_path_into(_dst: Option<&mut [u8]>, _name: Option<&[u8]>) -> i32 {
    todo!("platform-cocoa")
}

/// `os_get_config_path_ptr`, without the trailing NUL.
pub fn config_path_ptr(_name: Option<&[u8]>) -> Vec<u8> {
    todo!("platform-cocoa")
}

/// `os_get_program_data_path`.
pub fn program_data_path_into(_dst: Option<&mut [u8]>, _name: Option<&[u8]>) -> i32 {
    todo!("platform-cocoa")
}

/// `os_get_program_data_path_ptr`, without the trailing NUL.
pub fn program_data_path_ptr(_name: Option<&[u8]>) -> Vec<u8> {
    todo!("platform-cocoa")
}

/// `os_get_executable_path_ptr`, without the trailing NUL.
pub fn executable_path(_name: Option<&[u8]>) -> Option<Vec<u8>> {
    todo!("platform-cocoa")
}

/// `os_gettime_ns`.
pub fn gettime_ns() -> u64 {
    todo!("platform-cocoa")
}

/// `os_cpu_usage_info`.
pub struct CpuUsage {
    _private: (),
}

/// `os_cpu_usage_info_start`.
pub fn cpu_usage_start() -> Option<CpuUsage> {
    todo!("platform-cocoa")
}

/// `os_cpu_usage_info_query`.
pub fn cpu_usage_query(_info: Option<&mut CpuUsage>) -> f64 {
    todo!("platform-cocoa")
}

/// Retained `beginActivityWithOptions:` token.
pub struct PerformanceToken(*mut c_void);

impl PerformanceToken {
    /// The retained object pointer `os_request_high_performance` returns.
    pub fn into_raw(self) -> *mut c_void {
        let raw = self.0;
        let _ = raw;
        todo!("platform-cocoa")
    }

    /// `None` for a null token. Does not retain.
    pub fn from_raw(raw: *mut c_void) -> Option<Self> {
        let _ = raw;
        todo!("platform-cocoa")
    }
}

/// `os_request_high_performance`.
pub fn request_high_performance(_reason: Option<&[u8]>) -> Option<PerformanceToken> {
    todo!("platform-cocoa")
}

/// `os_end_high_performance`.
pub fn end_high_performance(_token: Option<PerformanceToken>) {
    todo!("platform-cocoa")
}

/// `os_inhibit_info`.
pub struct Inhibit {
    _private: (),
}

/// `os_inhibit_sleep_create`.
pub fn inhibit_create(_reason: Option<&[u8]>) -> Inhibit {
    todo!("platform-cocoa")
}

/// `os_inhibit_sleep_set_active`.
pub fn inhibit_set_active(_info: Option<&mut Inhibit>, _active: bool) -> bool {
    todo!("platform-cocoa")
}

/// `os_inhibit_sleep_destroy`.
pub fn inhibit_destroy(_info: Option<Inhibit>) {
    todo!("platform-cocoa")
}

/// `os_get_physical_cores`.
pub fn physical_cores() -> i32 {
    todo!("platform-cocoa")
}

/// `os_get_logical_cores`.
pub fn logical_cores() -> i32 {
    todo!("platform-cocoa")
}

/// `os_get_emulation_status`.
pub fn emulation_status() -> bool {
    todo!("platform-cocoa")
}

/// `os_get_sys_free_size`.
pub fn sys_free_size() -> u64 {
    todo!("platform-cocoa")
}

/// `os_get_free_space`.
pub fn free_space(_path: Option<&[u8]>) -> i64 {
    todo!("platform-cocoa")
}

/// `os_get_free_disk_space`.
pub fn free_disk_space(_path: Option<&[u8]>) -> u64 {
    todo!("platform-cocoa")
}

/// `os_get_sys_total_size`.
pub fn sys_total_size() -> u64 {
    todo!("platform-cocoa")
}

/// `os_get_proc_memory_usage` sizes.
pub struct ProcMem {
    /// Resident bytes.
    pub resident: u64,
    /// Virtual bytes.
    pub virtual_size: u64,
}

/// `os_get_proc_memory_usage`.
pub fn proc_memory() -> Option<ProcMem> {
    todo!("platform-cocoa")
}

/// `os_get_proc_resident_size`.
pub fn proc_resident_size() -> u64 {
    todo!("platform-cocoa")
}

/// `os_get_proc_virtual_size`.
pub fn proc_virtual_size() -> u64 {
    todo!("platform-cocoa")
}

/// `cfstr_copy_cstr`.
///
/// # Safety
/// `cfstring` is null or a live `CFStringRef` for the duration of the call.
pub unsafe fn copy_cstr(_cfstring: *const c_void, _encoding: u32) -> Option<Vec<u8>> {
    todo!("platform-cocoa")
}

/// `cfstr_copy_dstr`.
///
/// # Safety
/// `cfstring` is null or a live `CFStringRef` for the duration of the call.
pub unsafe fn copy_into_dstr(_cfstring: *const c_void, _encoding: u32, _dstr: &mut Dstr) -> bool {
    let _ = _dstr;
    todo!("platform-cocoa")
}
