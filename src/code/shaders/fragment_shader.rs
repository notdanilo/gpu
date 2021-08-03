use crate::prelude::*;
use crate::code::shaders::shader::Shader;
use crate::Context;

/// A fragment shader representation.
#[derive(Shrinkwrap)]
pub struct FragmentShader {
    shader : Shader
}

impl FragmentShader {
    /// Creates a new `FragmentShader` from a source code.
    pub fn new(context:&Context, source: &str) -> Result<Self, String> {
        // let shader = Shader::new(context, gl::FRAGMENT_SHADER, source)?;
        // Ok(Self{shader})
        unimplemented!()
    }
}