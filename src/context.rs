extern crate glutin;

pub use glutin::ContextError;
use glutin::ContextTrait;

pub enum ContextDisplay {
    None,
    Screen,
    Window(String, usize, usize)
}

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

    pub fn with_display(mut self, display: ContextDisplay) -> Self {
        self.display = display;
        self
    }

    pub fn cursor(mut self, cursor: bool) -> Self {
        self.cursor = cursor;
        self
    }

    pub fn vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn build(self) -> Context {
        let events_loop = glutin::EventsLoop::new();
        let mut window_builder = glutin::WindowBuilder::new();

        match &self.display {
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

        let context = match self.display {
            ContextDisplay::Window(_, _, _) | ContextDisplay::Screen => {
                glutin::ContextBuilder::new().with_vsync(self.vsync)
                    .build_windowed(window_builder, &events_loop)
                    .unwrap()
            },
            ContextDisplay::None => {
                glutin::ContextBuilder::new().with_vsync(self.vsync)
                    .build_windowed(window_builder, &events_loop) // the guideline for creating a headless context is: try build_headless, if it fails, fallback to hidden window
                    .unwrap()
            }
        };

        context.hide_cursor(!self.cursor);

        let gl = glow::Context::from_loader_function(|s| {
            context.get_proc_address(s) as *const _
        });

        Context {
            events_loop,
            context,
            gl
        }
    }
}

pub struct Context {
    events_loop : glutin::EventsLoop,
    context     : glutin::WindowedContext,
    pub gl      : glow::Context
}

impl Context {
    pub fn run(&mut self) -> bool {
        let events_loop = &mut self.events_loop;
        let context = &mut self.context;
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

    pub fn make_current(&self) -> Result<(), ContextError> {
        unsafe {
            self.context.make_current()
        }
    }

    pub fn swap_buffers(&self) -> Result<(), ContextError> {
        self.context.swap_buffers()
    }

    pub fn get_proc_address(&self, addr: &str) -> *const () {
        self.context.get_proc_address(addr)
    }

    pub fn inner_dimensions(&self) -> (usize, usize) {
        let dpi      = self.context.get_hidpi_factor();
        let logical  = self.context.get_inner_size().expect("Couldn't get inner size");
        let physical = logical.to_physical(dpi);
        (physical.width as usize, physical.height as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ContextBuilder, ContextDisplay};

    #[test]
    fn create_context() {
        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let mut context = context_builder.build();

        context.make_current().unwrap();
    }

    #[test]
    fn present_context() {
        use std::{thread, time};

        let context_builder = ContextBuilder::new().with_display(ContextDisplay::Window(String::from("present_context (black)"), 320, 240));
        let mut context = context_builder.build();

        context.make_current().unwrap();

        context.swap_buffers().unwrap();

        thread::sleep(time::Duration::from_millis(1000));
    }
}
