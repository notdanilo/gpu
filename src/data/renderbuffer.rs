use crate::prelude::*;
use crate::{Context, GLContext};

type RenderbufferResource = <glow::Context as HasContext>::Renderbuffer;

/// Renderbuffer representation.
pub struct Renderbuffer {
    gl       : GLContext,
    resource : RenderbufferResource
}

impl Renderbuffer {
    /// Creates a default `Renderbuffer`.
    pub fn default(context:&Context) -> Self {
        let resource = Default::default();
        let gl       = context.gl_context();
        Self { resource, gl }
    }

    /// Creates a new `Renderbuffer` with `(width, height)` dimensions.
    pub fn new(context:&Context, width: u32, height: u32) -> Self {
        let gl       = context.gl_context();
        let width    = width as i32;
        let height   = height as i32;
        let resource = unsafe {
            let resource = gl.create_renderbuffer().expect("Couldn't create Renderbuffer");
            gl.bind_renderbuffer(glow::RENDERBUFFER, Some(resource));
            gl.renderbuffer_storage(glow::RENDERBUFFER, glow::DEPTH_COMPONENT, width, height);
            resource
        };
        Self { gl, resource }
    }

    /// Gets the `RenderbufferResource`.
    pub fn resource(&self) -> RenderbufferResource {
        self.resource
    }
}

impl Drop for Renderbuffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_renderbuffer(self.resource());
        }
    }
}
