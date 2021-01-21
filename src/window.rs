pub type Size2 = (usize, usize);

pub struct OnResizeEvent {
    pub previous_size: Size2,
    pub size: Size2
}

impl OnResizeEvent {
    pub fn new(previous_size: Size2, size: Size2) -> Self {
        Self { previous_size, size }
    }
}

pub struct Window {
    title: String,
    size: (usize, usize),
    on_resize_callback: Option<Box<dyn FnMut(OnResizeEvent) + 'static>>
}

impl Window {
    pub fn new(name: String, size: Size2) -> Self {
        let on_resize_callback = None;
        Self { title: name, size, on_resize_callback }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn set_size(&mut self, size: Size2) {
        let old_size = self.size;
        self.size = size;
        self.on_resize_callback.as_mut().map(|callback| {
            (*callback)(OnResizeEvent::new(old_size, size))
        });
    }

    pub fn size(&self) -> Size2 {
        self.size
    }

    pub fn on_resize<Callback: FnMut(OnResizeEvent) + 'static>(&mut self, callback: Option<Callback>) {
        self.on_resize_callback = callback.map(|callback| Box::new(callback) as Box<dyn FnMut(OnResizeEvent)>);
    }
}
