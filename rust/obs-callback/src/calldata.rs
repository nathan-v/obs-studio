//! Safe calldata stack.
//!
//! The byte layout matches `libobs/callback/calldata.c.h`. Odd cases pinned
//! by `test/cmocka/test_calldata.c` (exact-fit growth, a null string stored
//! as size 0, a get that checks size rather than type) are characterized,
//! not endorsed. `bnum_allocs` stays with that C test: this core keeps an
//! owned buffer, and the shim is what calls `bmalloc`.

use std::marker::PhantomData;

/// `blog` text for a fixed stack that cannot grow. The shim logs this at
/// `LOG_ERROR` and does not store the write.
pub const FIXED_OVERFLOW_MESSAGE: &str = "Tried to go above fixed calldata stack size!";

/// Result of [`Calldata::get_string`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringGet<'a> {
    /// No parameter with that name. C leaves the out pointer unchanged.
    Missing,
    /// Stored as a zero-size parameter. C returns true and a null pointer.
    Null,
    /// Bytes in the stack, including the trailing NUL.
    Data(&'a [u8]),
}

/// Procedure-call parameter stack.
///
/// An owned stack grows. A fixed stack borrows a caller buffer and refuses
/// a write that does not fit.
pub struct Calldata<'a> {
    _marker: PhantomData<&'a mut [u8]>,
}

impl Calldata<'static> {
    /// Empty stack. Nothing is allocated until the first stored parameter.
    pub fn new() -> Self {
        todo!()
    }
}

impl<'a> Calldata<'a> {
    /// Borrow `buf` as a fixed stack. The first `size_t` is cleared.
    pub fn fixed(_buf: &'a mut [u8]) -> Self {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    /// Release an owned stack. A fixed stack is left untouched.
    pub fn free(&mut self) {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }

    pub fn capacity(&self) -> usize {
        todo!()
    }

    pub fn is_fixed(&self) -> bool {
        todo!()
    }

    /// The whole allocation, or `None` when nothing has been allocated.
    /// Bytes past [`Self::size`] are not defined.
    pub fn stack(&self) -> Option<&[u8]> {
        todo!()
    }

    /// How many fixed writes were refused. The shim logs each one.
    pub fn overflow_count(&self) -> u32 {
        todo!()
    }

    pub fn set_data(&mut self, _name: &str, _data: &[u8]) {
        todo!()
    }

    /// `Some` only when `name` is stored with exactly `size` bytes.
    pub fn get_data(&self, _name: &str, _size: usize) -> Option<&[u8]> {
        todo!()
    }

    pub fn get_string(&self, _name: &str) -> StringGet<'_> {
        todo!()
    }

    /// `None` for a missing name and for a null string. `Some("")` for an
    /// empty C string.
    pub fn string(&self, name: &str) -> Option<&str> {
        match self.get_string(name) {
            StringGet::Data(bytes) => {
                let end = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
                std::str::from_utf8(&bytes[..end]).ok()
            }
            StringGet::Missing | StringGet::Null => None,
        }
    }

    pub fn set_int(&mut self, name: &str, value: i64) {
        self.set_data(name, &value.to_ne_bytes());
    }

    pub fn get_int(&self, name: &str) -> Option<i64> {
        let bytes = self.get_data(name, std::mem::size_of::<i64>())?;
        Some(i64::from_ne_bytes(bytes.try_into().ok()?))
    }

    pub fn int(&self, name: &str) -> i64 {
        self.get_int(name).unwrap_or(0)
    }

    pub fn set_float(&mut self, name: &str, value: f64) {
        self.set_data(name, &value.to_ne_bytes());
    }

    pub fn get_float(&self, name: &str) -> Option<f64> {
        let bytes = self.get_data(name, std::mem::size_of::<f64>())?;
        Some(f64::from_ne_bytes(bytes.try_into().ok()?))
    }

    pub fn float(&self, name: &str) -> f64 {
        self.get_float(name).unwrap_or(0.0)
    }

    pub fn set_bool(&mut self, name: &str, value: bool) {
        self.set_data(name, &[u8::from(value)]);
    }

    pub fn get_bool(&self, name: &str) -> Option<bool> {
        let bytes = self.get_data(name, 1)?;
        Some(bytes[0] != 0)
    }

    pub fn bool(&self, name: &str) -> bool {
        self.get_bool(name).unwrap_or(false)
    }

    pub fn set_ptr(&mut self, name: &str, value: usize) {
        self.set_data(name, &value.to_ne_bytes());
    }

    pub fn get_ptr(&self, name: &str) -> Option<usize> {
        let bytes = self.get_data(name, std::mem::size_of::<usize>())?;
        Some(usize::from_ne_bytes(bytes.try_into().ok()?))
    }

    /// `0` when the name is missing. A stored null pointer is also `0`.
    pub fn ptr(&self, name: &str) -> usize {
        self.get_ptr(name).unwrap_or(0)
    }

    /// `None` stores a zero-size parameter. `Some` stores the characters
    /// plus a trailing NUL.
    pub fn set_string(&mut self, name: &str, value: Option<&str>) {
        match value {
            Some(text) => {
                let mut bytes = Vec::with_capacity(text.len() + 1);
                bytes.extend_from_slice(text.as_bytes());
                bytes.push(0);
                self.set_data(name, &bytes);
            }
            None => self.set_data(name, &[]),
        }
    }
}
