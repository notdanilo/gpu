use crate::data::Buffer;
use crate::data::Texture;
use crate::data::Sampler;

use crate::Resource;

pub trait Program {
    fn get_id(&self) -> u32;

    fn bind_buffer(&mut self, buffer: &Buffer, index: u32) {
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, index, buffer.get_id());
        }
    }
    fn bind_texture(&mut self, texture: &dyn Texture, sampler: &Sampler, index:
    u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
            gl::BindTexture(texture.get_type(), texture.get_id());
            gl::BindSampler(index, sampler.get_id());
            gl::UseProgram(self.get_id());
            gl::Uniform1i(index as i32, index as i32);
        }
    }
    fn bind_image(&mut self, texture: &dyn Texture, index: u32) {
        unsafe {
            gl::UseProgram(self.get_id());
            gl::ActiveTexture(gl::TEXTURE0 + index);
            gl::BindTexture(texture.get_type(), texture.get_id());
            gl::BindImageTexture(index, texture.get_id(), 0, gl::FALSE, 0, gl::WRITE_ONLY, texture.get_format().get_internal_format());
            gl::Uniform1i(index as i32, index as i32);
        }
    }
}
