//! Windows side of the [platform boundary](super): the recursive lock the
//! audio thread port takes.
//!
//! RED stub: `init` fails, so `audio_output_open` returns
//! `AUDIO_OUTPUT_FAIL` and the Tier 1 / Tier 3 tests that need an open
//! output fail. The GREEN commit replaces this with a `CRITICAL_SECTION`.

use core::ffi::c_int;

pub(crate) struct RecursiveMutex {
    _private: (),
}

// The real lock is shared across the audio thread and FFI callers. The
// stub never initializes, so nothing shares it.
unsafe impl Send for RecursiveMutex {}
unsafe impl Sync for RecursiveMutex {}

impl RecursiveMutex {
    pub(crate) fn uninit() -> Self {
        Self { _private: () }
    }

    /// # Safety
    /// Same contract as the Unix lock: call once, at the final address,
    /// before the mutex is shared.
    pub(crate) unsafe fn init(&mut self) -> Result<(), c_int> {
        let _ = self;
        Err(-1)
    }

    #[inline]
    pub(crate) fn lock(&self) {}

    #[inline]
    pub(crate) fn unlock(&self) {}
}
