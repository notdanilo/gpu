use crate::Resource;

pub struct Renderbuffer {
    id : u32
}

impl Renderbuffer {
    pub fn default() -> Self {
        Self {
            id : 0
        }
    }

    pub fn new(width: u32, height: u32) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenRenderbuffers(1, &mut id);
            gl::BindRenderbuffer(gl::RENDERBUFFER, id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width as i32, height as i32);
        }

        Self {
            id
        }
    }
}

impl Drop for Renderbuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.get_id());
        }
    }
}

impl Resource for Renderbuffer {
    fn get_id(&self) -> u32 { self.id }
}
