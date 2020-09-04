//use crate::data::Buffer;
//use crate::data::Texture;
//use crate::data::Sampler;

use crate::Context;
use glow::HasContext;

type ProgramResource = <glow::Context as HasContext>::Program;

/// A structure representing a GPU program.
pub struct Program<'context> {
    pub(crate) context : &'context Context,
    resource           : ProgramResource
}

impl<'context> Program<'context> {
    /// Creates a new `Program`.
    pub fn new(context:&'context Context) -> Self {
        let resource = unsafe {
            context.gl.create_program().expect("Couldn't create program")
        };
        Self {context,resource}
    }

    /// Gets the `ProgramResource` object.
    pub fn resource(&self) -> ProgramResource { self.resource }

// FIXME: These parts were removed because glow uses a minimum set of GL x GLES x WEBGL
//    fn bind_buffer(&mut self, buffer: &Buffer, index: u32) {
//        unsafe {
//            self::BindBufferBase(gl::SHADER_STORAGE_BUFFER, index, buffer.get_id());
//        }
//    }
//    fn bind_texture(&mut self, texture:&dyn Texture, sampler:&Sampler, index:u32) {
//        unsafe {
//            gl::ActiveTexture(gl::TEXTURE0 + index);
//            gl::BindTexture(texture.get_type(), texture.get_id());
//            gl::BindSampler(index, sampler.get_id());
//            gl::UseProgram(self.get_id());
//            gl::Uniform1i(index as i32, index as i32);
//        }
//    }
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

impl Drop for Program<'_> {
    fn drop(&mut self) {
        unsafe {
            self.context.gl.delete_program(self.resource());
        }
    }
}