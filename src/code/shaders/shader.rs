use crate::Context;
use glow::HasContext;

type ShaderResource = <glow::Context as HasContext>::Shader;

pub struct Shader<'context> {
    resource : ShaderResource,
    context  : &'context Context
}

impl<'context> Shader<'context> {
    pub fn new(context:&'context Context, shader_type:u32, source:&str) -> Result<Self, String> {
        let gl       = &context.gl;
        let resource = unsafe { gl.create_shader(shader_type)? };
        unsafe {
            gl.shader_source(resource, source);
            gl.compile_shader(resource);

            if !gl.get_shader_compile_status(resource) {
                return Err(gl.get_shader_info_log(resource));
            }
        }
        Ok(Self {resource,context})
    }

    pub fn resource(&self) -> ShaderResource { self.resource }
}

impl<'context> Drop for Shader<'context> {
    fn drop(&mut self) {
        unsafe {
            self.context.gl.delete_shader(self.resource());
        }
    }
}
