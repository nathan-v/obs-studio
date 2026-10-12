//! Windows stand-in for `os_sem_t` until the Win32 semaphore lands.
//! RED: [`Sem::try_new`] returns `None`, so Tier 1 fails on purpose.

use core::pin::Pin;

/// Empty semaphore. The real `CreateSemaphore` body replaces this.
pub struct Sem {
    _private: (),
}

impl Sem {
    /// `os_sem_init`. The stub never succeeds.
    pub fn try_new(_value: u32) -> Option<Pin<Box<Self>>> {
        None
    }
}
