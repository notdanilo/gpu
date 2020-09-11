use crate::prelude::*;
use crate::code::shaders::shader::Shader;
use crate::Context;

#[derive(Shrinkwrap)]
pub struct ComputeShader<'context> {
    shader : Shader<'context>
}

impl<'context> ComputeShader<'context> {
    pub fn new(context:&'context Context, source: &str) -> Result<Self, String> {
        let shader = Shader::new(context, glow::COMPUTE_SHADER, source)?;
        Ok(Self{shader})
    }
}