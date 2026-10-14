//! Windows half of `libobs/util/platform-windows.c`.
//!
//! RED stubs: every call returns the empty failure (`false`, `-1`, `None`,
//! `0`) so the Tier 1 tests fail an assertion. The GREEN commit replaces
//! these bodies.

#![allow(unused_variables)]

pub const MKDIR_SUCCESS: i32 = 0;
pub const MKDIR_EXISTS: i32 = 1;
pub const MKDIR_ERROR: i32 = -1;

pub struct DirEnt {
    pub name: Vec<u8>,
    pub directory: bool,
}

pub struct Dir {
    _private: (),
}

impl Dir {
    pub fn read(&mut self) -> Option<DirEnt> {
        None
    }
}

pub struct GlobEnt {
    pub path: Vec<u8>,
    pub directory: bool,
}

/// `os_file_exists`.
pub fn file_exists(path: &[u8]) -> bool {
    let _ = path;
    false
}

/// `os_unlink`. `0` or `-1`.
pub fn unlink(path: &[u8]) -> i32 {
    let _ = path;
    -1
}

/// `os_rmdir`.
pub fn rmdir(path: &[u8]) -> i32 {
    let _ = path;
    -1
}

/// `os_mkdir`. `0` created, `1` already exists, `-1` any other error.
pub fn mkdir(path: &[u8]) -> i32 {
    let _ = path;
    MKDIR_ERROR
}

/// `os_rename`.
pub fn rename(from: &[u8], to: &[u8]) -> i32 {
    let _ = (from, to);
    -1
}

/// `os_safe_replace`. A missing backup is `None`.
pub fn safe_replace(target: &[u8], from: &[u8], backup: Option<&[u8]>) -> i32 {
    let _ = (target, from, backup);
    -1
}

/// `os_copyfile`. Fails when the destination exists, as `CopyFileW` does
/// with `bFailIfExists`.
pub fn copy_file(from: &[u8], to: &[u8]) -> i32 {
    let _ = (from, to);
    -1
}

/// `os_opendir`.
pub fn opendir(path: &[u8]) -> Option<Dir> {
    let _ = path;
    None
}

/// `os_get_abs_path` of a path that fits `MAX_PATH`.
pub fn abs_path(path: &[u8]) -> Option<Vec<u8>> {
    let _ = path;
    None
}

/// `os_glob`. Paths are the bytes C stores in `os_globent.path`.
pub fn glob(pattern: &[u8]) -> Option<Vec<GlobEnt>> {
    let _ = pattern;
    None
}

/// `os_sleepto_ns`. False when the target is already past.
pub fn sleepto_ns(time_target: u64) -> bool {
    let _ = time_target;
    false
}

/// `os_sleepto_ns_fast`.
pub fn sleepto_ns_fast(time_target: u64) -> bool {
    let _ = time_target;
    false
}
