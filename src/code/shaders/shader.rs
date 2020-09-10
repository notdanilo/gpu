use crate::{Context, WeakContext};
use glow::HasContext;

type ShaderResource = <glow::Context as HasContext>::Shader;

/// A shader representation.
pub struct Shader {
    resource : ShaderResource,
    context  : WeakContext
}

impl Shader {
    /// Creates a new `Shader`.
    pub fn new(context:&Context, shader_type:u32, source:&str) -> Result<Self, String> {
        let gl       = &context.data.borrow().gl;
        let resource = unsafe { gl.create_shader(shader_type)? };
        unsafe {
            gl.shader_source(resource, source);
            gl.compile_shader(resource);

            if !gl.get_shader_compile_status(resource) {
                return Err(gl.get_shader_info_log(resource));
            }
        }
        let context = context.weak();
        Ok(Self {resource,context})
    }

    /// Gets the `ShaderResource`.
    pub fn resource(&self) -> ShaderResource { self.resource }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.context.upgrade().map(|context| {
                context.data.borrow().gl.delete_shader(self.resource());
            });
        }
    }
}
