use crate::Context;
use glow::HasContext;

type RenderbufferResource = <glow::Context as HasContext>::Renderbuffer;

/// Renderbuffer representation.
pub struct Renderbuffer<'context> {
    context  : &'context Context,
    resource : RenderbufferResource
}

impl<'context> Renderbuffer<'context> {
    /// Creates a default `Renderbuffer`.
    pub fn default(context:&'context Context) -> Self {
        let resource = Default::default();
        Self {resource,context}
    }

    /// Creates a new `Renderbuffer` with `(width, height)` dimensions.
    pub fn new(context:&'context Context, width: u32, height: u32) -> Self {
        let gl       = &context.gl;
        let width    = width as i32;
        let height   = height as i32;
        let resource = unsafe {
            let resource = gl.create_renderbuffer().expect("Couldn't create Renderbuffer");
            gl.bind_renderbuffer(glow::RENDERBUFFER, Some(resource));
            gl.renderbuffer_storage(glow::RENDERBUFFER, glow::DEPTH_COMPONENT, width, height);
            resource
        };
        Self {context,resource}
    }

    /// Gets the `RenderbufferResource`.
    pub fn resource(&self) -> RenderbufferResource {
        self.resource
    }
}

impl<'context> Drop for Renderbuffer<'context> {
    fn drop(&mut self) {
        unsafe {
            self.context.gl.delete_renderbuffer(self.resource());
        }
    }
}
