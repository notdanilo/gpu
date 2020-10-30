//use crate::data::Buffer;
//use crate::data::Texture;
//use crate::data::Sampler;

use crate::{Context, GLContext, Sampler, Texture2D};

type ProgramResource = u32;

/// A structure representing a GPU program.
pub struct Program {
    pub(crate) gl : GLContext,
    resource      : ProgramResource
}

impl Program {
    /// Creates a new `Program`.
    pub fn new(context: &Context) -> Self {
        let gl = context.gl_context();
        let resource = unsafe {
            gl::CreateProgram()
        };
        Self { gl, resource}
    }

    /// Gets the `ProgramResource` object.
    pub fn resource(&self) -> ProgramResource { self.resource }

    pub fn uniform_mat4(&mut self, location: usize, transpose: bool, v: &[f32]) {
        unsafe {
            gl::UniformMatrix4fv(location as i32, 1, transpose as u8, v.as_ptr());
        }
    }

// FIXME: These parts were removed because glow uses a minimum set of GL x GLES x WEBGL.
// These functions can be included in a trait which can be implemented for backends that supports it.
//    fn bind_buffer(&mut self, buffer: &Buffer, index: u32) {
//        unsafe {
//            self::BindBufferBase(gl::SHADER_STORAGE_BUFFER, index, buffer.get_id());
//        }
//    }

    //FIXME: Rename to bind_sampler?
    /// Binds a `Texture2D` at `index` so it can be sampled with the specified `sampler`,
    pub fn bind_texture_2d(&mut self, texture:&Texture2D, sampler:&Sampler, index: usize) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index as u32);
            gl::BindTexture(gl::TEXTURE_2D, texture.resource());
            gl::BindSampler(index as u32, sampler.resource());
            gl::UseProgram(self.resource());
            gl::Uniform1i(index as i32, index as i32);
        }
    }

    pub fn bind_vec2(&mut self, vec2:(f32, f32), index: usize) {
        unsafe {
            gl::Uniform2f(index as i32, vec2.0, vec2.1);
        }
    }

   pub fn bind_image_2d(&mut self, texture: &Texture2D, index: usize) {
       unsafe {
           gl::UseProgram(self.resource());
           gl::ActiveTexture(gl::TEXTURE0 + index as u32);
           gl::BindTexture(gl::TEXTURE_2D, texture.resource());
           gl::BindImageTexture(index as u32, texture.resource(), 0, gl::FALSE, 0, gl::READ_WRITE , texture.format().internal_format());
           gl::Uniform1i(index as i32, index as i32);
       }
   }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.resource());
        }
    }
}