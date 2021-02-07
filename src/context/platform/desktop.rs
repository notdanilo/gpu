use crate::{ContextBuilder, ContextDisplay, HasContext, HasGLContext, GLContext, OnResizeEvent};

pub use glutin::ContextError;
use glutin::ContextTrait;


// ===============
// === Context ===
// ===============

/// GPU `Context` representation.
pub struct Context {
    display     : ContextDisplay,
    events_loop : glutin::EventsLoop,
    context     : glutin::WindowedContext,
    gl          : GLContext
}

impl HasGLContext for Context {
    fn gl_context(&self) -> GLContext {
        self.gl.clone()
    }
}

impl HasContext for Context {
    fn new(builder: ContextBuilder) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let mut window_builder = glutin::WindowBuilder::new();

        match &builder.display {
            ContextDisplay::Window(window) => {
                window_builder = window_builder.with_title(window.title())
                    .with_dimensions(glutin::dpi::LogicalSize::new(window.size().0 as f64, window.size().1 as f64));
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
            ContextDisplay::Window(_) | ContextDisplay::Screen => {
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

        gl::load_with(|s| {
            context.get_proc_address(s) as *const _
        });

        let gl = GLContext {};
        Self { display: builder.display, events_loop, context, gl }
    }

    fn display(&self) -> &ContextDisplay {
        &self.display
    }

    fn display_mut(&mut self) -> &mut ContextDisplay {
        &mut self.display
    }

    fn run(&mut self) -> bool {
        let events_loop = &mut self.events_loop;
        let context = &mut self.context;
        let display = &mut self.display;
        let mut available = true;
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::CloseRequested => available = false,
                        glutin::WindowEvent::Resized(logical_size) => {
                            let dpi_factor = context.get_hidpi_factor();
                            let size = logical_size.to_physical(dpi_factor);
                            context.resize(size);
                            if let ContextDisplay::Window(window) = display {
                                window.set_size((size.width as usize, size.height as usize));
                            }
                        },
                        glutin::WindowEvent::CursorMoved {device_id, modifiers, position} => {
                            let dpi_factor = context.get_hidpi_factor();
                            let position = position.to_physical(dpi_factor);
                            println!("Cursor {}x{}", position.x, position.y);
                        },
                        glutin::WindowEvent::KeyboardInput {device_id,input} => {
                            input.virtual_keycode.map(|vk| {

                            });
                        }
                        _ => ()
                    }
                },
                _ => ()
            }
        });
        available
    }

    fn make_current(&self) -> Result<(), ContextError> {
        unsafe {
            self.context.make_current()

        }
    }

    fn swap_buffers(&self) -> Result<(), ContextError> {
        self.context.swap_buffers()
    }

    fn get_proc_address(&self, addr: &str) -> *const () {
        self.context.get_proc_address(addr)
    }

    fn resolution(&self) -> (usize, usize) {
        let dpi      = self.context.get_hidpi_factor();
        let logical  = self.context.get_inner_size().expect("Couldn't get inner size");
        let physical = logical.to_physical(dpi);
        (physical.width as usize, physical.height as usize)
    }
}