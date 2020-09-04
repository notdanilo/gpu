use crate::prelude::*;
use crate::code::shaders::shader::Shader;
use crate::Context;

/// A vertex shader representation.
#[derive(Shrinkwrap)]
pub struct VertexShader<'context> {
    shader : Shader<'context>
}

impl<'context> VertexShader<'context> {
    /// Creates a new `FragmentShader` from a source code.
    pub fn new(context:&'context Context, source: &str) -> Result<Self, String> {
        let shader = Shader::new(context, glow::VERTEX_SHADER, source)?;
        Ok(Self{shader})
    }
}