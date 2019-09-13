use crate::code::shaders::shader::create_shader;
use crate::Resource;

pub struct ComputeShader {
    id : u32
}

impl ComputeShader {
    pub fn new(source: &str) -> Result<Self, String> {
        let id = create_shader(gl::COMPUTE_SHADER, &source);
        match id {
            Ok(id) => Ok(Self{ id }),
            Err(err) => Err(err)
        }
    }
}

impl Drop for ComputeShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.get_id());
        }
    }
}

impl Resource for ComputeShader {
    fn get_id(&self) -> u32 {
        self.id
    }
}
