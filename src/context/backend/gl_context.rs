//! GLContext backend.

use std::rc::Rc;

/// A `GLContext` structure.
#[derive(Clone,shrinkwraprs::Shrinkwrap)]
pub struct GLContext {
    /// A `GLContext` with `glow::Context`.
    pub gl : Rc<glow::Context>
}

impl GLContext {
    /// Creates a new `GLContext` from `glow::Context`.
    pub fn from_glow_context(glow: glow::Context) -> Self {
        GLContext { gl: Rc::new(glow) }
    }
}

/// A trait used for retrieving a `GLContext`.
pub trait HasGLContext {
    /// Gets the `GLContext`.
    fn gl_context(&self) -> GLContext;
}
