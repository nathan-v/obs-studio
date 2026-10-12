//! POSIX half of `libobs/util/platform-nix.c`.
//!
//! RED stubs. The GREEN port replaces these bodies.

/// `os_file_exists`.
pub fn file_exists(path: &[u8]) -> bool {
    let _ = path;
    todo!("os_file_exists")
}

/// `os_unlink`. Returns 0 or -1, as `unlink(2)` does.
pub fn unlink(path: &[u8]) -> i32 {
    let _ = path;
    todo!("os_unlink")
}

/// `os_rmdir`.
pub fn rmdir(path: &[u8]) -> i32 {
    let _ = path;
    todo!("os_rmdir")
}

/// `os_mkdir`. `0` created, `1` already exists, `-1` any other error.
pub fn mkdir(path: &[u8]) -> i32 {
    let _ = path;
    todo!("os_mkdir")
}

/// `os_rename`.
pub fn rename(from: &[u8], to: &[u8]) -> i32 {
    let _ = (from, to);
    todo!("os_rename")
}

/// `os_safe_replace`. A missing `backup` skips the backup rename.
pub fn safe_replace(target: &[u8], from: &[u8], backup: Option<&[u8]>) -> i32 {
    let _ = (target, from, backup);
    todo!("os_safe_replace")
}

/// One `os_readdir` result, with `.` and `..` still included.
pub struct DirEnt {
    /// `d_name` without the NUL.
    pub name: Vec<u8>,
    /// `directory`.
    pub directory: bool,
}

/// `os_opendir` / `os_readdir` / `os_closedir`.
pub struct Dir {
    _private: (),
}

/// `os_opendir`. `None` when `opendir` fails.
pub fn opendir(path: &[u8]) -> Option<Dir> {
    let _ = path;
    todo!("os_opendir")
}

impl Dir {
    /// `os_readdir`. `None` at the end, or when the name does not fit in
    /// `d_name[256]` (that entry is consumed, as in C).
    pub fn read(&mut self) -> Option<DirEnt> {
        todo!("os_readdir")
    }
}

/// `os_get_abs_path_ptr` as an owned path. `None` when `realpath` fails.
pub fn abs_path(path: &[u8]) -> Option<Vec<u8>> {
    let _ = path;
    todo!("os_get_abs_path_ptr")
}
