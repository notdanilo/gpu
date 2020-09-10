use crate::{ContextBuilder, ContextDisplay, GPUContext};

pub use glutin::ContextError;
use glutin::ContextTrait;
use std::rc::{Rc, Weak};
use std::cell::RefCell;



// ===============
// === Context ===
// ===============

pub struct ContextData {
    events_loop   : glutin::EventsLoop,
    context       : glutin::WindowedContext,
    // TODO: We want more backend support like Vulkan.
    pub(crate) gl : glow::Context
}

pub struct WeakContext {
    pub(crate) data : Weak<RefCell<ContextData>>
}

impl WeakContext {
    pub fn upgrade(&self) -> Option<Context> {
        self.data.upgrade().map(|data| {
            Context { data }
        })
    }
}

/// GPU `Context` representation.
pub struct Context {
    pub data : Rc<RefCell<ContextData>>
}

impl GPUContext for Context {
    /// Creates a new `Context`.
    fn new(builder:&ContextBuilder) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let mut window_builder = glutin::WindowBuilder::new();

        match &builder.display {
            ContextDisplay::Window(name, width, height) => {
                window_builder = window_builder.with_title(name)
                    .with_dimensions(glutin::dpi::LogicalSize::new(*width as f64, *height as f64));
            },
            ContextDisplay::Screen => {
                window_builder = window_builder.with_title("")
                    .with_fullscreen(Some(events_loop.get_primary_monitor()));
            },
            ContextDisplay::None => {
                window_builder = window_builder.with_title("")
                    .with_fullscreen(Some(events_loop.get_primary_monitor()))
                    .with_visibility(false);
            }
        }

        let context = match builder.display {
            ContextDisplay::Window(_, _, _) | ContextDisplay::Screen => {
                glutin::ContextBuilder::new().with_vsync(builder.vsync)
                    .build_windowed(window_builder, &events_loop)
                    .unwrap()
            },
            ContextDisplay::None => {
                glutin::ContextBuilder::new().with_vsync(builder.vsync)
                    .build_windowed(window_builder, &events_loop) // the guideline for creating a headless context is: try build_headless, if it fails, fallback to hidden window
                    .unwrap()
            }
        };

        context.hide_cursor(!builder.cursor);

        unsafe {
            context.make_current().expect("Context make current failed.");
        }

        let gl = glow::Context::from_loader_function(|s| {
            context.get_proc_address(s) as *const _
        });

        let data = Rc::new(RefCell::new(ContextData{events_loop,context,gl}));
        Self { data }
    }

    /// Runs the `Context` and returns `false` if the `Context` is no longer available.
    fn run(&mut self) -> bool {
        use std::ops::DerefMut;
        let mut data = self.data.borrow_mut();
        let data = data.deref_mut();
        let events_loop = &mut data.events_loop;
        let context = &mut data.context;
        let mut available = true;
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent{ event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => available = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = context.get_hidpi_factor();
                        context.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => ()
                }
            }
        });
        available
    }

    /// Makes the `Context` current for the current thread.
    fn make_current(&self) -> Result<(), ContextError> {
        unsafe {
            self.data.borrow().context.make_current()

        }
    }

    /// Swap buffers for presenting in the `ContextDisplay`.
    fn swap_buffers(&self) -> Result<(), ContextError> {
        self.data.borrow().context.swap_buffers()
    }

    /// OpenGL function dynamic loading.
    fn get_proc_address(&self, addr: &str) -> *const () {
        self.data.borrow().context.get_proc_address(addr)
    }

    /// Gets the inner dimensions of the `ContextDisplay`.
    fn inner_dimensions(&self) -> (usize, usize) {
        let dpi      = self.data.borrow().context.get_hidpi_factor();
        let logical  = self.data.borrow().context.get_inner_size().expect("Couldn't get inner size");
        let physical = logical.to_physical(dpi);
        (physical.width as usize, physical.height as usize)
    }
}

impl Context {
    pub fn weak(&self) -> WeakContext {
        WeakContext { data: Rc::downgrade(&self.data) }
    }
}
