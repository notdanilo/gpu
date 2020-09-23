//use crate::data::Buffer;
//use crate::data::Texture;
//use crate::data::Sampler;

use crate::prelude::*;
use crate::{Context, GLContext, Sampler, Texture2D};

type ProgramResource = <glow::Context as HasContext>::Program;

/// A structure representing a GPU program.
pub struct Program {
    pub(crate) gl      : GLContext,
    resource           : ProgramResource
}

impl Program {
    /// Creates a new `Program`.
    pub fn new(context: &Context) -> Self {
        let gl = context.gl_context();
        let resource = unsafe {
            gl.create_program().expect("Couldn't create program")
        };
        Self { gl, resource}
    }

    /// Gets the `ProgramResource` object.
    pub fn resource(&self) -> ProgramResource { self.resource }

    pub fn uniform_mat4(&mut self, location: usize, transpose: bool, v: &[f32]) {
        unsafe {
            self.gl.gl.uniform_matrix_4_f32_slice(Some(location as u32).as_ref(), transpose, v);
        }
    }

// FIXME: These parts were removed because glow uses a minimum set of GL x GLES x WEBGL.
// These functions can be included in a trait which can be implemented for backends that supports it.
//    fn bind_buffer(&mut self, buffer: &Buffer, index: u32) {
//        unsafe {
//            self::BindBufferBase(gl::SHADER_STORAGE_BUFFER, index, buffer.get_id());
//        }
//    }

    /// Binds a `Texture2D` at `index` so it can be sampled with the specified `sampler`,
    pub fn bind_texture_2d(&mut self, texture:&Texture2D, sampler:&Sampler, index:u32) {
        unsafe {
            self.gl.active_texture(glow::TEXTURE0 + index);
            self.gl.bind_texture(glow::TEXTURE_2D, Some(texture.resource()));
            self.gl.bind_sampler(index, Some(sampler.resource()));
            self.gl.use_program(Some(self.resource()));
            self.gl.uniform_1_i32(Some(&index), index as i32);
        }
    }

    pub fn bind_vec2(&mut self, vec2:(f32, f32), index:u32) {
        unsafe {
            self.gl.uniform_2_f32(Some(&index), vec2.0, vec2.1);
        }
    }

    //TODO: Unify bind_texture_2d with bind_image.

//    fn bind_image(&mut self, texture: &dyn Texture, index: u32) {
//        unsafe {
//            gl::UseProgram(self.get_id());
//            gl::ActiveTexture(gl::TEXTURE0 + index);
//            gl::BindTexture(texture.get_type(), texture.get_id());
//            gl::BindImageTexture(index, texture.get_id(), 0, gl::FALSE, 0, gl::WRITE_ONLY, texture.get_format().get_internal_format());
//            gl::Uniform1i(index as i32, index as i32);
//        }
//    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.resource());
        }
    }
}