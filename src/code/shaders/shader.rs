use std::ffi::CString;
use std::str;
use std::ptr;
use crate::Context;
use glow::HasContext;

use crate::Resource;

pub trait Shader : Resource {}

pub fn create_shader(context:&Context, shader_type:u32, source:&str) -> Result<u32, String> {
    let gl = &context.gl;
    let id = unsafe { gl.create_shader(shader_type)? };
    unsafe {
        gl.shader_source(id, source);
        gl.compile_shader(id);


        if !gl.get_shader_compile_status(id) {
            return Err(gl.get_shader_info_log(id));
        }
    }
    Ok(id)
}
