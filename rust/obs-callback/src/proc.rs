//! Safe procedure table.
//!
//! `libobs/callback/proc.c.h` stores callbacks under the name parsed from a
//! declaration. This core stores the name bytes and the callback address.
//! Parsing, `blog`, and the call itself stay in the shim: a Tier 1 test
//! cannot invoke a raw function pointer. `bnum_allocs` and a null handler
//! stay with `test/cmocka/test_proc.c`.

/// What [`ProcHandler::add`] did with a name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddOutcome {
    /// The name was not in the table. The callback is stored.
    Added,
    /// The name was already stored. The first callback stays.
    Exists,
}

/// A lookup that has not called the procedure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreparedCall {
    /// No procedure has that name.
    Missing,
    /// Copied callback address and data. The caller invokes it.
    Ready { func: usize, data: usize },
}

/// Name to callback table. The first registration of a name wins.
pub struct ProcHandler {
    _private: (),
}

impl ProcHandler {
    pub fn new() -> Self {
        todo!("proc handler")
    }

    /// Register `func` under `name`. A second add of the same bytes is
    /// [`AddOutcome::Exists`] and does not replace the first callback.
    pub fn add(&self, name: &[u8], func: usize, data: usize) -> AddOutcome {
        let _ = (name, func, data);
        todo!("proc handler add")
    }

    /// Copy the callback for `name` and release the table lock before
    /// returning. `None` is missing: the shim is what faults in `strcmp`
    /// once a procedure is registered.
    pub fn prepare_call(&self, name: Option<&[u8]>) -> PreparedCall {
        let _ = name;
        todo!("proc handler call")
    }
}
