/// 2D size type.
pub type Size2D = (usize, usize);

/// Resize event.
pub struct OnResizeEvent {
    /// Previous size.
    pub previous_size: Size2D,
    /// New size.
    pub size: Size2D
}

impl OnResizeEvent {
    /// Create a new resize event.
    pub fn new(previous_size: Size2D, size: Size2D) -> Self {
        Self { previous_size, size }
    }
}

/// Window.
pub struct Window {
    title: String,
    size: (usize, usize),
    on_resize_callback: Option<Box<dyn FnMut(OnResizeEvent) + 'static>>
}

impl Window {
    /// Create a new Window.
    pub fn new(name: String, size: Size2D) -> Self {
        let on_resize_callback = None;
        Self { title: name, size, on_resize_callback }
    }

    /// Set Window title.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Get Window title.
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Set Window size.
    pub fn set_size(&mut self, size: Size2D) {
        let old_size = self.size;
        self.size = size;
        self.on_resize_callback.as_mut().map(|callback| {
            (*callback)(OnResizeEvent::new(old_size, size))
        });
    }

    /// Get Window size.
    pub fn size(&self) -> Size2D {
        self.size
    }

    /// Register a on resize callback.
    pub fn on_resize<Callback: FnMut(OnResizeEvent) + 'static>(&mut self, callback: Option<Callback>) {
        self.on_resize_callback = callback.map(|callback| Box::new(callback) as Box<_>);
    }
}
