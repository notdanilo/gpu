use crate::Resource;

pub struct Sampler {
    id : u32
}

impl Sampler {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenSamplers(1, &mut id);
            gl::SamplerParameteri(id, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::SamplerParameteri(id, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::SamplerParameteri(id, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::SamplerParameteri(id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }
        Self {
            id : id
        }
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteSamplers(1, &mut self.id);
        }
    }
}

impl Resource for Sampler {
    fn get_id(&self) -> u32 { self.id }
}
