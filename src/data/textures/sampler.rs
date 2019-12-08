use crate::Resource;

pub struct Sampler {
    id : u32
}

impl Default for Sampler {
    fn default() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenSamplers(1, &mut id);
            gl::SamplerParameteri(id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::SamplerParameteri(id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::SamplerParameteri(id, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::SamplerParameteri(id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }
        Self {
            id
        }
    }
}

impl Sampler {
    pub fn new() -> Self { Default::default() }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteSamplers(1, &self.get_id());
        }
    }
}

impl Resource for Sampler {
    fn get_id(&self) -> u32 { self.id }
}
