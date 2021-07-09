use crate::{ContextBuilder, IsContext, IsGLContext, GLContext, Framebuffer};

pub use glutin::ContextError;
use surface::Surface;

pub trait ContextBuilderExt {
    fn build_raw_from_surface(self, surface: &Surface) -> Result<glutin::RawContext<glutin::NotCurrent>, String>;
}

#[cfg(any(
target_os = "linux",
target_os = "dragonfly",
target_os = "freebsd",
target_os = "netbsd",
target_os = "openbsd"
))]
pub mod unix;

// ===============
// === Context ===
// ===============

/// GPU `Context` representation.
pub struct Context {
    surface     : Option<Surface>,
    context     : glutin::RawContext<glutin::PossiblyCurrent>,
    gl          : GLContext,
    framebuffer : Option<Framebuffer>
}

impl IsGLContext for Context {
    fn gl_context(&self) -> GLContext {
        self.gl.clone()
    }

    fn get_proc_address(&self, addr: &str) -> *const std::ffi::c_void {
        self.context.get_proc_address(addr)
    }
}

impl IsContext for Context {
    fn framebuffer(&self) -> Option<&Framebuffer> {
        self.framebuffer.as_ref()
    }

    fn framebuffer_mut(&mut self) -> Option<&mut Framebuffer> {
        self.framebuffer.as_mut()
    }

    fn new(builder: ContextBuilder) -> Result<Self, String> {
        let surface = builder.surface.expect("Headless context still not supported.");
        let context = glutin::ContextBuilder::new()
            .with_vsync(builder.vsync)
            .with_double_buffer(Some(true))
            .build_raw_from_surface(&surface)?;
        let context = unsafe {
            context.make_current().map_err(|_| "Couldn't make current.".to_string())?
        };
        let framebuffer = None;
        let gl = GLContext {};
        let context = Self { surface: Some(surface), framebuffer, context, gl };
        Ok(context)
        // let context = match builder.surface {
        //     ContextDisplay::Window(_) | ContextDisplay::Screen => {
        //         glutin::ContextBuilder::new().with_vsync(builder.vsync)
        //             .build_windowed(window_builder, &events_loop)
        //             .unwrap()
        //     },
        //     ContextDisplay::None => {
        //         glutin::ContextBuilder::new().with_vsync(builder.vsync)
        //             .build_windowed(window_builder, &events_loop) // the guideline for creating a headless context is: try build_headless, if it fails, fallback to hidden surface
        //             .unwrap()
        //     }
        // };
        //
        // context.hide_cursor(!builder.cursor);
        //
        // unsafe {
        //     context.make_current().expect("Context make current failed.");
        // }
        //
        // gl::load_with(|s| {
        //     context.get_proc_address(s) as *const _
        // });
        //
        // let gl = GLContext {};
        // Self { display: builder.surface, events_loop, context, gl }
    }

    fn surface(&self) -> Option<&Surface> {
        self.surface.as_ref()
    }

    fn surface_mut(&mut self) -> Option<&mut Surface> {
        self.surface.as_mut()
    }

    fn swap_buffers(&mut self) -> Result<(), ContextError> {
        self.context.swap_buffers()
    }
}