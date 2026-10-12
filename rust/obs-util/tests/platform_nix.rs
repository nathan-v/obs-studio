//! Tier 1: safe-core tests for `libobs/util/platform-nix.c`.
//!
//! Each test names the cmocka case in `test/cmocka/test_platform.c` it
//! mirrors. Cases that call `platform.c` (`os_mkdirs`, `os_fopen`, the
//! quick-read helpers) are not this unit.

#![cfg(unix)]

use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use obs_util::platform::unix::nix::{self, DirEnt};

fn bytes(path: &Path) -> &[u8] {
    path.as_os_str().as_bytes()
}

fn scratch() -> std::path::PathBuf {
    static N: AtomicU64 = AtomicU64::new(0);
    let id = N.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("obs-platform-nix-{}-{}", std::process::id(), id));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

struct Tmp(std::path::PathBuf);

impl Tmp {
    fn new() -> Self {
        Self(scratch())
    }

    fn path(&self, name: &str) -> std::path::PathBuf {
        self.0.join(name)
    }
}

impl Drop for Tmp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Mirrors `test_file_exists` in `test/cmocka/test_platform.c`.
#[test]
fn test_file_exists() {
    let tmp = Tmp::new();
    let path = tmp.path("exists.txt");
    assert!(!nix::file_exists(bytes(&path)));
    fs::write(&path, "x").unwrap();
    assert!(nix::file_exists(bytes(&path)));
    assert_eq!(nix::unlink(bytes(&path)), 0);
    assert!(!nix::file_exists(bytes(&path)));
}

/// Mirrors `test_unlink_missing_fails`.
#[test]
fn test_unlink_missing_fails() {
    let tmp = Tmp::new();
    let path = tmp.path("missing");
    assert_eq!(nix::unlink(bytes(&path)), -1);
}

/// Mirrors `test_mkdir_success_then_exists`.
#[test]
fn test_mkdir_success_then_exists() {
    let tmp = Tmp::new();
    let path = tmp.path("mk");
    assert_eq!(nix::mkdir(bytes(&path)), 0);
    assert_eq!(nix::mkdir(bytes(&path)), 1);
    assert_eq!(nix::rmdir(bytes(&path)), 0);
    assert!(!nix::file_exists(bytes(&path)));
}

/// Mirrors `test_mkdir_missing_parent_is_error`.
#[test]
fn test_mkdir_missing_parent_is_error() {
    let tmp = Tmp::new();
    let path = tmp.path("no_parent").join("child");
    assert_eq!(nix::mkdir(bytes(&path)), -1);
}

/// Mirrors `test_rename`.
#[test]
fn test_rename() {
    let tmp = Tmp::new();
    let a = tmp.path("a.txt");
    let b = tmp.path("b.txt");
    fs::write(&a, "payload").unwrap();
    assert_eq!(nix::rename(bytes(&a), bytes(&b)), 0);
    assert!(!nix::file_exists(bytes(&a)));
    assert!(nix::file_exists(bytes(&b)));
    assert_eq!(fs::read(&b).unwrap(), b"payload");
    assert_eq!(nix::rename(bytes(&a), bytes(&b)), -1);
}

/// Mirrors `test_safe_replace_with_backup`.
#[test]
fn test_safe_replace_with_backup() {
    let tmp = Tmp::new();
    let target = tmp.path("target");
    let from = tmp.path("from");
    let bak = tmp.path("bak");
    fs::write(&target, "old").unwrap();
    fs::write(&from, "new").unwrap();
    assert_eq!(
        nix::safe_replace(bytes(&target), bytes(&from), Some(bytes(&bak))),
        0
    );
    assert_eq!(fs::read(&target).unwrap(), b"new");
    assert_eq!(fs::read(&bak).unwrap(), b"old");
    assert!(!nix::file_exists(bytes(&from)));
}

/// Mirrors `test_safe_replace_without_backup`.
#[test]
fn test_safe_replace_without_backup() {
    let tmp = Tmp::new();
    let target = tmp.path("target");
    let from = tmp.path("from");
    let bak = tmp.path("bak");
    fs::write(&target, "old").unwrap();
    fs::write(&from, "new").unwrap();
    assert_eq!(nix::safe_replace(bytes(&target), bytes(&from), None), 0);
    assert_eq!(fs::read(&target).unwrap(), b"new");
    assert!(!nix::file_exists(bytes(&from)));
    assert!(!bak.exists());
}

/// Mirrors `test_opendir_readdir_closedir`.
#[test]
fn test_opendir_readdir_closedir() {
    let tmp = Tmp::new();
    let dir = tmp.path("list");
    let sub = dir.join("sub");
    fs::create_dir(&dir).unwrap();
    fs::create_dir(&sub).unwrap();
    fs::write(dir.join("l1.txt"), "1").unwrap();
    fs::write(dir.join("l2.txt"), "2").unwrap();

    let mut handle = nix::opendir(bytes(&dir)).unwrap();
    let mut ents: Vec<DirEnt> = Vec::new();
    while let Some(ent) = handle.read() {
        if ent.name == b"." || ent.name == b".." {
            continue;
        }
        ents.push(ent);
    }
    assert_eq!(ents.len(), 3);
    ents.sort_by(|a, b| a.name.cmp(&b.name));
    assert_eq!(ents[0].name, b"l1.txt");
    assert!(!ents[0].directory);
    assert_eq!(ents[1].name, b"l2.txt");
    assert!(!ents[1].directory);
    assert_eq!(ents[2].name, b"sub");
    assert!(ents[2].directory);

    assert!(nix::opendir(bytes(&tmp.path("no_such_dir"))).is_none());
}

/// Mirrors `test_get_abs_path_ptr_dot`.
#[test]
fn test_get_abs_path_ptr_dot() {
    let path = nix::abs_path(b".").unwrap();
    assert!(!path.is_empty());
    assert_eq!(path[0], b'/');
}
