use std::ffi::CString;
use std::str;
use std::ptr;
use gl::types::*;

use crate::Resource;

pub trait Shader : Resource {}

pub fn create_shader(shader_type : u32, source: &str) -> Result<u32, String> {
    let id = unsafe { gl::CreateShader(shader_type) };
    unsafe {
        let c_str = CString::new(source.as_bytes()).unwrap();

        gl::ShaderSource(id, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(id);

        let mut success = i32::from(gl::FALSE);
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

        if success != i32::from(gl::TRUE) {
            let mut len = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let mut info_log = Vec::with_capacity(len as usize);
            info_log.set_len((len as usize) - 1); // -1 to skip trialing null character

            gl::GetShaderInfoLog(id, len, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            return Err(String::from_utf8(info_log).unwrap());
        }
    }
    Ok(id)
}
