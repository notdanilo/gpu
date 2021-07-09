use super::{Context, BackendContext, IsContext};
use surface::Surface;

// FIXME: We no longer need ContextBuilder as it would only require us to pass Option<Surface> to it.
//  We could define Context::new(surface: Option<Surface>) instead.

/// A builder for `Context`.
pub struct ContextBuilder {
    pub(super) vsync   : bool,
    pub(super) surface : Option<Surface>
}

impl Default for ContextBuilder {
    fn default() -> Self {
        let surface = None;
        let vsync = true;
        Self { vsync, surface }
    }
}

impl ContextBuilder {
    /// Creates a new `ContextBuilder` with the default parameters:
    /// cursor = false
    /// vsync  = true
    /// surface = None
    pub fn new() -> Self { Default::default() }

    /// Sets the surface of the `Context`.
    pub fn with_surface(mut self, surface: Option<Surface>) -> Self {
        self.surface = surface;
        self
    }

    /// Sets vertical synchronization.
    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    /// Creates a new `Context` with all the parameters specified in the `ContextBuilder`.
    pub fn build(self) -> Result<Context, String> {
        // FIXME: Why can't I just result.map(Box::new)?
        match BackendContext::new(self) {
            Ok(context) => Ok(Box::new(context)),
            Err(e) => Err(e)
        }
    }
}