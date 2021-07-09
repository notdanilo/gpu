use glutin::platform::unix::{
    EventLoopWindowTargetExtUnix, RawContextExt, WindowExtUnix,
};

use super::ContextBuilderExt;
use crate::Surface;

impl ContextBuilderExt for glutin::ContextBuilder<'_, glutin::NotCurrent> {
    fn build_raw_from_surface(self, surface: &Surface) -> Result<glutin::RawContext<glutin::NotCurrent>, String> {
        let window = &surface.window;
        if surface.event_loop.is_wayland() {
            let size = window.inner_size();
            let (width, height): (u32, u32) = size.into();
            let display_ptr = window.wayland_display().unwrap() as *const _;
            let surface = window.wayland_surface().unwrap();

            let raw_context = unsafe {
                self
                    .build_raw_wayland_context(display_ptr, surface, width, height)
                    .expect("Couldn't build raw wayland context.")
            };
            Ok(raw_context)
        } else {
            let xconn = surface.event_loop.xlib_xconnection().expect("Couldn't get xconnection.");
            let xwindow = window.xlib_window().expect("Couldn't get xwindow");
            println!("{:?} {:?}", xconn, xwindow);
            let raw_context = unsafe {
                self
                    .build_raw_x11_context(xconn, xwindow)
                    .expect("Couldn't build raw x11 context.")
            };
            Ok(raw_context)
        }
    }
}