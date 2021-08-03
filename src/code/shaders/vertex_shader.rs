use crate::prelude::*;
use crate::code::shaders::shader::Shader;
use crate::Context;

/// A vertex shader representation.
#[derive(Shrinkwrap)]
pub struct VertexShader {
    shader : Shader
}

impl VertexShader {
    /// Creates a new `FragmentShader` from a source code.
    pub fn new(context:&Context, source: &str) -> Result<Self, String> {
        // let shader = Shader::new(context, gl::VERTEX_SHADER, source)?;
        // Ok(Self{shader})
        unimplemented!()
    }
}