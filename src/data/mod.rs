extern crate gl;

pub use crate::Resource;

mod buffer;
mod textures;
mod vao;
mod renderbuffer;
mod framebuffer;

pub use buffer::Buffer;
pub use textures::*;
pub use vao::VertexArrayObject;
pub use renderbuffer::Renderbuffer;
pub use framebuffer::Framebuffer;
