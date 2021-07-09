//! GLContext backend.

/// A `GLContext` structure.
#[derive(Clone,Copy)]
pub struct GLContext {
}

impl GLContext {
}

/// A trait used for retrieving a `GLContext`.
pub trait IsGLContext {
    /// Gets the `GLContext`.
    fn gl_context(&self) -> GLContext;

    /// OpenGL extensions' function loader.
    fn get_proc_address(&self, addr: &str) -> *const std::ffi::c_void;
}
