use crate::code::shaders::shader::Shader;
use crate::Context;

use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap)]
pub struct FragmentShader<'context> {
    shader : Shader<'context>
}

impl<'context> FragmentShader<'context> {
    pub fn new(context:&'context Context, source: &str) -> Result<Self, String> {
        let shader = Shader::new(context, glow::FRAGMENT_SHADER, source)?;
        Ok(Self{shader})
    }
}