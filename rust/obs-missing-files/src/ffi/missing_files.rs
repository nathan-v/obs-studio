//! C ABI for `libobs/obs-missing-files.c.h`. Stubs until the port lands.

use std::ffi::{c_char, c_int, c_void};

pub type MissingFileCb = Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void)>;

/// # Safety
/// Same contract as C `obs_missing_files_create`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn obs_missing_files_create() -> *mut c_void {
    todo!()
}

/// # Safety
/// Same contract as C `obs_missing_files_destroy`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn obs_missing_files_destroy(files: *mut c_void) {
    let _ = files;
    todo!()
}

/// # Safety
/// Same contract as C `obs_missing_files_add_file`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn obs_missing_files_add_file(files: *mut c_void, file: *mut c_void) {
    let _ = (files, file);
    todo!()
}

/// # Safety
/// Same contract as C `obs_missing_files_count`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn obs_missing_files_count(files: *mut c_void) -> usize {
    let _ = files;
    todo!()
}

/// # Safety
/// Same contract as C `obs_missing_files_get_file`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn obs_missing_files_get_file(files: *mut c_void, idx: c_int) -> *mut c_void {
    let _ = (files, idx);
    todo!()
}

/// # Safety
/// Same contract as C `obs_missing_files_append`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn obs_missing_files_append(dst: *mut c_void, src: *mut c_void) {
    let _ = (dst, src);
    todo!()
}

/// # Safety
/// Same contract as C `obs_missing_file_create`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn obs_missing_file_create(
    path: *const c_char,
    callback: MissingFileCb,
    src_type: c_int,
    src: *mut c_void,
    data: *mut c_void,
) -> *mut c_void {
    let _ = (path, callback, src_type, src, data);
    todo!()
}

/// # Safety
/// Same contract as C `obs_missing_file_release`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn obs_missing_file_release(file: *mut c_void) {
    let _ = file;
    todo!()
}

/// # Safety
/// Same contract as C `obs_missing_file_destroy`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn obs_missing_file_destroy(file: *mut c_void) {
    let _ = file;
    todo!()
}

/// # Safety
/// Same contract as C `obs_missing_file_issue_callback`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn obs_missing_file_issue_callback(
    file: *mut c_void,
    new_path: *const c_char,
) {
    let _ = (file, new_path);
    todo!()
}

/// # Safety
/// Same contract as C `obs_missing_file_get_path`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn obs_missing_file_get_path(file: *mut c_void) -> *const c_char {
    let _ = file;
    todo!()
}

/// # Safety
/// Same contract as C `obs_missing_file_get_source_name`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn obs_missing_file_get_source_name(file: *mut c_void) -> *const c_char {
    let _ = file;
    todo!()
}
