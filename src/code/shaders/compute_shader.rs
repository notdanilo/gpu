use crate::prelude::*;
use crate::code::shaders::shader::Shader;
use crate::Context;

/// A compute shader representation.
#[derive(Shrinkwrap)]
pub struct ComputeShader {
    shader : Shader
}

impl ComputeShader {
    /// Creates a new `ComputeShader` from a source code.
    pub fn new(context:&Context, source: &str) -> Result<Self, String> {
        let shader = Shader::new(context, gl::COMPUTE_SHADER, source)?;
        Ok(Self{shader})
    }
}