//! Context creation module.

mod context_display;
pub use context_display::ContextDisplay;

mod context_builder;
pub use context_builder::ContextBuilder;

pub trait ContextInternals {
    fn internal_context(&self) -> Rc<glow::Context>;
}

pub trait GPUContext: ContextInternals {
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

use std::rc::{ Rc, Weak };
use std::cell::RefCell;

pub struct WeakContext {
    context : Weak<RefCell<dyn GPUContext>>
}

impl WeakContext {
    pub fn upgrade(&self) -> Option<Context> {
        self.context.upgrade().map(|context| {
            Context { context }
        })
    }
}

pub struct Context {
    context : Rc<RefCell<dyn GPUContext>>
}

impl Context {
    pub fn new(builder:&ContextBuilder) -> Self {
        Self { context : Rc::new(RefCell::new(BackendContext::new(builder))) }
    }

    pub fn weak_ref(&self) -> WeakContext {
        WeakContext { context : Rc::downgrade(&self.context) }
    }
}

impl ContextInternals for Context {
    fn internal_context(&self) -> Rc<glow::Context> {
        self.context.borrow().internal_context()
    }
}

impl GPUContext for Context {
    fn run(&mut self) -> bool {
        self.context.borrow_mut().run()
    }

    fn make_current(&self) -> Result<(), ContextError> {
        self.context.borrow().make_current()
    }

    fn swap_buffers(&self) -> Result<(), ContextError> {
        self.context.borrow().swap_buffers()
    }

    fn get_proc_address(&self, addr: &str) -> *const () {
        self.context.borrow().get_proc_address(addr)
    }

    fn inner_dimensions(&self) -> (usize, usize) {
        self.context.borrow().inner_dimensions()
    }
}

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