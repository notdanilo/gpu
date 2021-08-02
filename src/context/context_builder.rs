use super::{Context, BackendContext, HasContext};
use surface::Surface;

/// A builder for `Context`.
pub struct ContextBuilder {
    pub(super) cursor  : bool,
    pub(super) vsync   : bool,
    pub(super) surface : Option<Surface>
}

impl Default for ContextBuilder {
    fn default() -> Self {
        let cursor  = false;
        let vsync   = true;
        let surface = None;
        Self {cursor, vsync, surface}
    }
}

impl ContextBuilder {
    /// Creates a new `ContextBuilder` with the default parameters:
    /// cursor = false
    /// vsync  = true
    /// display = ContextDisplay::None
    pub fn new() -> Self { Default::default() }

    /// Sets the surface of the `Context`.
    pub fn with_surface(mut self, surface: Option<Surface>) -> Self {
        self.surface = surface;
        self
    }

    // FIXME: This doesn't make sense for a context without a display.
    /// Sets if we want a cursor for the created `Context`.
    pub fn with_cursor(mut self, cursor:bool) -> Self {
        self.cursor = cursor;
        self
    }

    /// Sets if we want vsync for the created `Context`.
    pub fn with_vsync(mut self, vsync:bool) -> Self {
        self.vsync = vsync;
        self
    }

    /// Creates a new `Context` with all the parameters specified in the `ContextBuilder`.
    pub fn build(self) -> Context {
        Box::new(BackendContext::new(&self))
    }

    #[cfg(target_arch = "wasm32")]
    /// Creates a new `Context` from a `web_sys::HtmlCanvasElement` with all the parameters
    /// specified in the `ContextBuilder`.
    pub fn build_from_canvas(self, canvas: web_sys::HtmlCanvasElement) -> Context {
        Box::new(BackendContext::from_canvas(canvas))
    }
}