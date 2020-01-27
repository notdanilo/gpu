use crate::code::shaders::shader::Shader;
use crate::Context;

use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap)]
pub struct VertexShader<'context> {
    shader : Shader<'context>
}

impl<'context> VertexShader<'context> {
    pub fn new(context:&'context Context, source: &str) -> Result<Self, String> {
        let shader = Shader::new(context, glow::VERTEX_SHADER, source)?;
        Ok(Self{shader})
    }
}