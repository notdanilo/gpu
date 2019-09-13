use crate::code::shaders::shader::create_shader;
use crate::Resource;

pub struct VertexShader {
    id : u32
}

impl VertexShader {
    pub fn new(source: &str) -> Result<Self, String> {
        let id = create_shader(gl::VERTEX_SHADER, &source);
        match id {
            Ok(id) => Ok(Self{ id }),
            Err(err) => Err(err)
        }
    }
}

impl Drop for VertexShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.get_id());
        }
    }
}

impl Resource for VertexShader {
    fn get_id(&self) -> u32 {
        self.id
    }
}
