use crate::Framebuffer;

use crate::Context;
use glow::HasContext;

pub struct ClearProgram<'context> {
    context : &'context Context,
    color : (f32, f32, f32, f32),
    depth: f64,
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

    pub fn set_depth(&mut self, depth: f64) { self.depth = depth; }
    pub fn depth(&self) -> f64 { self.depth }

    pub fn set_stencil(&mut self, stencil: i32) { self.stencil = stencil; }
    pub fn stencil(&self) -> i32 { self.stencil }

    pub fn clear(&self, framebuffer:&'context mut Framebuffer<'_>, mask: u32) {
        let gl = &self.context.gl;
        unsafe {
            gl.clear_color(self.color.0, self.color.1, self.color.2, self.color.3);
            gl.clear_depth_f64(self.depth);
            gl.clear_stencil(self.stencil);
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer.resource()));
            gl.clear(mask);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ContextBuilder, ContextDisplay, ClearProgram, Framebuffer};

    #[test]
    fn clear_display() {
        use std::{thread, time};

        let components = 3;
        let dimension = (320, 240);

        let context_builder = ContextBuilder::new().with_display(ContextDisplay::Window
            (String::from("clear_display (white)"), dimension.0, dimension.1));
        let mut context = context_builder.build();

        context.make_current().unwrap();

        let mut framebuffer = Framebuffer::default(&context);

        let mut clear_program = ClearProgram::new(&context);
        clear_program.set_color((1.0, 1.0, 1.0, 1.0));
        clear_program.clear(&mut framebuffer, ClearProgram::COLOR);

        context.swap_buffers().unwrap();

        thread::sleep(time::Duration::from_millis(1000));
    }
}