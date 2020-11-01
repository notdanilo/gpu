//! GLContext backend.

/// A `GLContext` structure.
#[derive(Clone,Copy)]
pub struct GLContext {
}

impl GLContext {
}

/// A trait used for retrieving a `GLContext`.
pub trait HasGLContext {
    /// Gets the `GLContext`.
    fn gl_context(&self) -> GLContext;
}
