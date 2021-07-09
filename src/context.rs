//! Context creation module.

mod context_builder;
pub use context_builder::ContextBuilder;

pub mod backend;
pub(crate) use backend::gl_context::{GLContext, IsGLContext};

pub use surface::Surface;

/// A trait defining the `GPUContext` interface.
pub trait IsContext: IsGLContext {
    /// Creates a new `Context`.
    fn new(builder: ContextBuilder) -> Result<Self, String> where Self: Sized;

    /// Gets a reference to the Context's Surface.
    fn surface(&self) -> Option<&Surface>;

    /// Gets a mutable reference to the Context's Surface.
    fn surface_mut(&mut self) -> Option<&mut Surface>;

    /// Swap buffers for presenting in the `ContextDisplay`.
    fn swap_buffers(&mut self) -> Result<(), ContextError>;

    /// Gets a reference to the default Framebuffer, if present.
    fn framebuffer(&self) -> Option<&Framebuffer>;

    /// Gets a mutable reference to the default Framebuffer, if present.
    fn framebuffer_mut(&mut self) -> Option<&mut Framebuffer>;
}

/// The `Context` object.
pub type Context = Box<dyn IsContext>;

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
use crate::Framebuffer;