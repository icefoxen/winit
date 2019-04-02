#![cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd",
           target_os = "netbsd", target_os = "openbsd"))]

pub use self::event_loop::{EventsLoop, EventsLoopProxy, EventsLoopSink, MonitorId};
pub use self::window::Window;

use sctk::reexports::client::protocol::wl_surface;
use sctk::reexports::client::Proxy;

mod event_loop;
mod keyboard;
mod pointer;
mod touch;
mod window;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId;

impl DeviceId {
    pub unsafe fn dummy() -> Self {
        DeviceId
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(usize);

impl WindowId {
    pub unsafe fn dummy() -> Self {
        WindowId(0)
    }
}

#[inline]
fn make_wid(s: &Proxy<wl_surface::WlSurface>) -> WindowId {
    WindowId(s.c_ptr() as usize)
}
