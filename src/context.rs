//! Context creation module.

mod context_display;
pub use context_display::ContextDisplay;

mod context_builder;
pub use context_builder::ContextBuilder;

pub mod backend;
pub(crate) use backend::gl_context::{GLContext, HasGLContext};

/// A trait defining the `GPUContext` interface.
pub trait HasContext: HasGLContext {
    /// Creates a new `Context`.
    fn new(builder: &ContextBuilder) -> Self where Self: Sized;

    /// Runs the `Context` and returns `false` if the `Context` is no longer available.
    fn run(&mut self) -> bool;

    /// Makes the `Context` current for the current thread.
    fn make_current(&self) -> Result<(), ContextError>;

    /// Swap buffers for presenting in the `ContextDisplay`.
    fn swap_buffers(&self) -> Result<(), ContextError>;

    /// OpenGL function dynamic loading.
    fn get_proc_address(&self, addr: &str) -> *const ();

    /// Gets the inner dimensions of the `ContextDisplay`.
    fn inner_dimensions(&self) -> (usize, usize);
}

/// The `Context` object.
pub type Context = Box<dyn HasContext>;

#[cfg(not(all(target_arch = "wasm32")))]
mod platform {
    mod desktop;
    pub use desktop::Context as BackendContext;
    pub use desktop::ContextError;
}

#[cfg(all(target_arch = "wasm32"))]
mod platform {
    mod web;
    pub use web::Context as BackendContext;
    pub use web::ContextError;
}

/// The Context object.
pub use platform::*;