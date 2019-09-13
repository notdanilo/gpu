use crate::code::shaders::shader::create_shader;
use crate::Resource;

pub struct FragmentShader {
    id : u32
}

impl FragmentShader {
    pub fn new(source: &str) -> Result<Self, String> {
        let id = create_shader(gl::FRAGMENT_SHADER, &source);
        match id {
            Ok(id) => Ok(Self{ id }),
            Err(err) => Err(err)
        }
    }
}

impl Drop for FragmentShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.get_id());
        }
    }
}

impl Resource for FragmentShader {
    fn get_id(&self) -> u32 {
        self.id
    }
}
