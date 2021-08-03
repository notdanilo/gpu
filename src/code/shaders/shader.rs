use crate::{Context, GLContext};

type ShaderResource = u32;

/// A shader representation.
pub struct Shader {
    resource : ShaderResource,
    _gl: GLContext
}

impl Shader {
    /// Creates a new `Shader`.
    pub fn new(context: &Context, shader_type:u32, source:&str) -> Result<Self, String> {
        // let gl       = context.gl_context();
        // let resource = unsafe { gl::CreateShader(shader_type) };
        // unsafe {
        //     gl::ShaderSource(resource, 1, &(source.as_ptr() as *const i8), &(source.len() as i32));
        //     gl::CompileShader(resource);
        //
        //     let mut compile_status = gl::FALSE as i32;
        //     gl::GetShaderiv(resource, gl::COMPILE_STATUS, &mut compile_status);
        //     if compile_status == gl::FALSE as i32 {
        //         let buffer_size = 4096;
        //         let mut length = 0;
        //         let mut buffer : [u8; 4096] = [0; 4096];
        //         gl::GetShaderInfoLog(resource, buffer_size, &mut length, buffer.as_mut_ptr() as *mut i8);
        //         let err = String::from_raw_parts(buffer.as_mut_ptr(), length as usize, buffer_size as usize);
        //         return Err(err)
        //     }
        // }
        // Ok(Self {resource, _gl: gl })
        unimplemented!()
    }

    /// Gets the `ShaderResource`.
    pub fn resource(&self) -> ShaderResource { self.resource }
}

impl Drop for Shader {
    fn drop(&mut self) {
        // unsafe {
        //     gl::DeleteShader(self.resource());
        // }
        unimplemented!()
    }
}
