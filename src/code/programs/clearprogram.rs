use crate::Framebuffer;
use crate::Resource;

pub struct ClearProgram {
    color : (f32, f32, f32, f32),
    depth: f64,
    stencil: i32
}

impl ClearProgram {
    pub const COLOR: u32 = gl::COLOR_BUFFER_BIT;
    pub const DEPTH: u32 = gl::DEPTH_BUFFER_BIT;
    pub const STENCIL: u32 = gl::STENCIL_BUFFER_BIT;

    pub fn new() -> ClearProgram {
        ClearProgram {
            color: (0.0, 0.0, 0.0, 0.0),
            depth: 1.0, // is it default?
            stencil: 0 // is it default?
        }
    }

    pub fn set_color(&mut self, color: (f32, f32, f32, f32)) { self.color = color; }
    pub fn get_color(&self) -> (f32, f32, f32, f32) { self.color }

    pub fn set_depth(&mut self, depth: f64) { self.depth = depth; }
    pub fn get_depth(&self) -> f64 { self.depth }

    pub fn set_stencil(&mut self, stencil: i32) { self.stencil = stencil; }
    pub fn get_stencil(&self) -> i32 { self.stencil }

    pub fn clear(&self, framebuffer: &Framebuffer, mask: u32) {
        unsafe {
            gl::ClearColor(self.color.0, self.color.1, self.color.2, self.color.3);
            gl::ClearDepth(self.depth);
            gl::ClearStencil(self.stencil);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer.get_id());
            gl::Clear(mask);
        }
    }
}
