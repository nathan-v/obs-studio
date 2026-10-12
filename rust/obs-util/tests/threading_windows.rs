//! Tier 1 for `util/threading-windows.c`: the semaphore and event cases
//! from `test/cmocka/test_threading.c`. RED against empty stubs.

#![cfg(windows)]

use std::ptr;

use obs_c_oracle as _; // links the oracle test stubs (blog, bcrash, bmem)
use obs_util::ffi::threading::os_event_init;
use obs_util::threading::Sem;

/// test_sem_initial_value
#[test]
fn sem_initial_value() {
    let sem = Sem::try_new(2);
    assert!(
        sem.is_some(),
        "os_sem_init(2) returned no semaphore (empty stub)"
    );
}

/// test_event_init_auto_manual, the auto half
#[test]
fn event_init_auto() {
    let mut event: *mut core::ffi::c_void = ptr::null_mut();
    let code = unsafe { os_event_init(&mut event, 0) };
    assert_eq!(code, 0, "os_event_init returned {code} (empty stub)");
    assert!(!event.is_null());
}
