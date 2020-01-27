use crate::Framebuffer;

use crate::Context;
use glow::HasContext;

pub struct ClearProgram<'context> {
    context : &'context Context,
    color : (f32, f32, f32, f32),
    depth: f32,
    stencil: i32
}

impl<'context> ClearProgram<'context> {
    pub const COLOR   : u32 = glow::COLOR_BUFFER_BIT;
    pub const DEPTH   : u32 = glow::DEPTH_BUFFER_BIT;
    pub const STENCIL : u32 = glow::STENCIL_BUFFER_BIT;

    pub fn new(context:&'context Context) -> Self {
        let color = (0.0, 0.0, 0.0, 0.0);
        let depth = 1.0; // is it default?
        let stencil = 0; // is it default?
        Self {context,color,depth,stencil}
    }

    pub fn set_color(&mut self, color: (f32, f32, f32, f32)) { self.color = color; }
    pub fn color(&self) -> (f32, f32, f32, f32) { self.color }

    pub fn set_depth(&mut self, depth: f32) { self.depth = depth; }
    pub fn depth(&self) -> f32 { self.depth }

    pub fn set_stencil(&mut self, stencil: i32) { self.stencil = stencil; }
    pub fn stencil(&self) -> i32 { self.stencil }

    pub fn clear(&self, framebuffer:&'context mut Framebuffer<'_>, mask: u32) {
        let gl = &self.context.gl;
        unsafe {
            framebuffer.bind();
            gl.clear_color(self.color.0, self.color.1, self.color.2, self.color.3);
            gl.clear_depth_f32(self.depth);
            gl.clear_stencil(self.stencil);
            gl.clear(mask);
        }
    }
}