extern crate glutin;

pub use glutin::ContextError;
use glutin::ContextTrait;

pub struct Context {
    events_loop : glutin::EventsLoop,
    context : glutin::WindowedContext
}

impl Context {
    pub fn new() -> Context {
        let events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new()
            .with_fullscreen(Some(events_loop.get_primary_monitor()))
            .with_title("slxrev19");

        let context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &events_loop)
            .unwrap();

        context.hide_cursor(true);

        Context {
            events_loop : events_loop,
            context : context
        }
    }

    pub fn run(&mut self) -> bool {
        let events_loop = &mut self.events_loop;
        let context = &mut self.context;
        let mut available = true;
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => available = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                       let dpi_factor = context.get_hidpi_factor();
                       context.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => ()
                },
                _ => ()
            }
        });
        available
    }

    pub fn make_current(&mut self) -> Result<(), ContextError> {
        unsafe {
            self.context.make_current()
        }
    }

    pub fn swap_buffers(&mut self) -> Result<(), ContextError> {
        self.context.swap_buffers()
    }

    pub fn get_proc_address(&self, addr: &str) -> *const () {
        self.context.get_proc_address(addr)
    }
}

#[test]
fn create_context() {
    let mut context = Context::new();

    context.make_current().unwrap();
    crate::initialize(|symbol| context.get_proc_address(symbol) as *const _);
}

#[test]
fn present_context() {
    let mut context = Context::new();

    context.make_current().unwrap();
    crate::initialize(|symbol| context.get_proc_address(symbol) as *const _);

    context.swap_buffers().unwrap();
}
