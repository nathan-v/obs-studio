//! Safe core of `libobs/util/platform-nix-portal.c.h` (#1631).
//!
//! GIO stays behind [`Host`]. The session bus, the Inhibit method, the
//! Response signal and Close are asynchronous in C; the host stores the
//! callbacks and the tests (or the C ABI shim) run them.

use crate::base::{LOG_DEBUG, LOG_ERROR, LOG_WARNING};

/// `G_BUS_TYPE_SESSION`.
pub const SESSION_BUS: i32 = 2;
/// Sleep and idle, the flags `platform-nix-portal.c` always sends.
pub const INHIBIT_FLAGS: u32 = 0xC;
pub const PORTAL_NAME: &str = "org.freedesktop.portal.Desktop";
pub const PORTAL_PATH: &str = "/org/freedesktop/portal/desktop";
pub const INHIBIT_IFACE: &str = "org.freedesktop.portal.Inhibit";
pub const REQUEST_IFACE: &str = "org.freedesktop.portal.Request";
pub const PROPERTIES_IFACE: &str = "org.freedesktop.DBus.Properties";
pub const CALL_FLAGS_NONE: i32 = 0;
pub const CALL_TIMEOUT_INFINITE: i32 = -1;
/// `G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE`.
pub const SIGNAL_FLAGS_NO_MATCH_RULE: u32 = 1;

/// How `g_dbus_connection_call_finish` ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallResult {
    /// A reply, including a Response of 0.
    Ok,
    /// A D-Bus error other than cancellation.
    Failed,
    /// `G_IO_ERROR_CANCELLED`.
    Cancelled,
}

/// What the portal client asks the desktop bus to do.
pub trait Host {
    type Conn;

    fn bus_get(&self, bus_type: i32) -> Result<Self::Conn, String>;
    fn unique_name(&self, conn: &Self::Conn) -> String;
    fn properties_get(&self, conn: &Self::Conn, interface: &str, property: &str) -> bool;
    fn request_id(&self) -> u32;
    fn subscribe(
        &self,
        conn: &Self::Conn,
        sender: &str,
        interface: &str,
        member: &str,
        path: &str,
        flags: u32,
        on_response: Box<dyn Fn(u32) + Send>,
    ) -> u32;
    fn unsubscribe(&self, conn: &Self::Conn, id: u32);
    fn inhibit(
        &self,
        conn: &Self::Conn,
        destination: &str,
        path: &str,
        interface: &str,
        method: &str,
        call_flags: i32,
        timeout: i32,
        window: &str,
        inhibit_flags: u32,
        token: &str,
        reason: &str,
        on_done: Box<dyn Fn(CallResult) + Send>,
    );
    fn close(
        &self,
        conn: &Self::Conn,
        destination: &str,
        path: &str,
        interface: &str,
        method: &str,
        call_flags: i32,
        timeout: i32,
        on_done: Box<dyn Fn(CallResult) + Send>,
    );
    fn cancel(&self);
    fn log(&self, level: i32, message: &str);
}

/// `struct portal_inhibit_info`.
pub struct Portal<H: Host> {
    _host: H,
}

impl<H: Host> Portal<H> {
    /// `portal_inhibit_info_create`.
    pub fn create(_host: H) -> Option<Self> {
        let _ = (LOG_DEBUG, LOG_ERROR, LOG_WARNING);
        todo!()
    }

    /// `portal_inhibit_info_destroy`. `None` is the C NULL handle.
    pub fn destroy(_portal: Option<Self>) {
        todo!()
    }

    pub fn is_active(&self) -> bool {
        todo!()
    }

    pub fn request_path(&self) -> Option<String> {
        todo!()
    }

    pub fn subscribed(&self) -> bool {
        todo!()
    }

    pub fn has_cancellable(&self) -> bool {
        todo!()
    }

    /// `portal_inhibit`.
    pub fn set_active(&self, _reason: &str, _active: bool) {
        todo!()
    }
}
