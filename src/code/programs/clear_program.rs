use crate::prelude::*;
use crate::{Context, Framebuffer, GLContext};



// ====================
// === ClearProgram ===
// ====================

/// A program that clears colors, depth and stencil of a `framebuffer`.
pub struct ClearProgram {
    gl: GLContext,
    color: (f32, f32, f32, f32),
    depth: f32,
    stencil: i32
}

impl ClearProgram {
    /// Color buffer bit.
    pub const COLOR   : u32 = glow::COLOR_BUFFER_BIT;
    /// Depth buffer bit.
    pub const DEPTH   : u32 = glow::DEPTH_BUFFER_BIT;
    /// Stencil buffer bit.
    pub const STENCIL : u32 = glow::STENCIL_BUFFER_BIT;

    /// Creates a new `ClearProgram`.
    pub fn new(context:&Context) -> Self {
        let gl = context.gl_context();
        let color = (0.0, 0.0, 0.0, 0.0);
        let depth = 1.0; // FIXME: is it default?
        let stencil = 0; // FIXME: is it default?
        Self { gl, color, depth, stencil}
    }

    /// Sets the color clear value.
    pub fn set_color(&mut self, color: (f32, f32, f32, f32)) { self.color = color; }
    /// Gets the color clear value.
    pub fn color(&self) -> (f32, f32, f32, f32) { self.color }

    /// Sets the depth clear value.
    pub fn set_depth(&mut self, depth: f32) { self.depth = depth; }
    /// Gets the depth clear value.
    pub fn depth(&self) -> f32 { self.depth }

    /// Sets the stencil clear value.
    pub fn set_stencil(&mut self, stencil: i32) { self.stencil = stencil; }
    /// Gets the stencil clear value.
    pub fn stencil(&self) -> i32 { self.stencil }

    /// Clear the target `Framebuffer` using the buffer bit mask.
    /// Here is an example that clears color, depth and stencil in a single call:
    /// ```rust,ignore
    /// clear(framebuffer, ClearProgram::COLOR | ClearProgram::DEPTH | ClearProgram::STENCIL)
    /// ```
    pub fn clear(&self, framebuffer:&mut Framebuffer, clear_mask: u32) {
        let gl = &self.gl;
        unsafe {
            framebuffer.bind();
            gl.clear_color(self.color.0, self.color.1, self.color.2, self.color.3);
            gl.clear_depth_f32(self.depth);
            gl.clear_stencil(self.stencil);
            gl.clear(clear_mask);
        }
    }
}