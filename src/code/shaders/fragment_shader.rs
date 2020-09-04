use crate::prelude::*;
use crate::code::shaders::shader::Shader;
use crate::Context;

/// A fragment shader representation.
#[derive(Shrinkwrap)]
pub struct FragmentShader<'context> {
    shader : Shader<'context>
}

impl<'context> FragmentShader<'context> {
    /// Creates a new `FragmentShader` from a source code.
    pub fn new(context:&'context Context, source: &str) -> Result<Self, String> {
        let shader = Shader::new(context, glow::FRAGMENT_SHADER, source)?;
        Ok(Self{shader})
    }
}