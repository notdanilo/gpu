#[cfg(not(all(target_arch = "wasm32")))]
mod platform {
    mod desktop;
    pub use desktop::Context;
}

#[cfg(all(target_arch = "wasm32"))]
mod platform {
    mod web;
    pub use web::Context;
}

pub use platform::Context;

// ======================
// === ContextDisplay ===
// ======================

pub enum ContextDisplay {
    None,
    Screen,
    Window(String, usize, usize)
}



// ======================
// === ContextBuilder ===
// ======================

pub struct ContextBuilder {
    cursor  : bool,
    vsync   : bool,
    display : ContextDisplay
}

impl Default for ContextBuilder {
    fn default() -> Self {
        let cursor  = false;
        let vsync   = true;
        let display = ContextDisplay::Screen;
        Self {cursor,vsync,display}
    }
}

impl ContextBuilder {
    pub fn new() -> Self { Default::default() }

    pub fn with_display(mut self, display:ContextDisplay) -> Self {
        self.display = display;
        self
    }

    pub fn cursor(mut self, cursor:bool) -> Self {
        self.cursor = cursor;
        self
    }

    pub fn vsync(mut self, vsync:bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn build(self) -> Context {
        Context::new(&self)
    }
}