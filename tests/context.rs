mod utils;

#[cfg(test)]
mod context {
    use super::utils::test;
    use gpu::ContextBuilder;
    use gpu::Context;
    use gpu::Surface;
    use surface::{Window, SurfaceBuilder, SurfaceContext};

    #[test]
    fn create_context() {
        let context = ContextBuilder::new().build().expect("Couldn't create context.");
        assert!(context.framebuffer().is_none());
        assert!(context.surface().is_none());
    }

    #[test]
    fn present_context() {
        let window = Window::new("Present Context.".into());
        let surface_context = SurfaceContext::Window(window);
        let surface = SurfaceBuilder::new(surface_context).build().expect("Couldn't create surface");
        let mut context = ContextBuilder::new().with_surface(Some(surface)).build().expect("Couldn't create context.");
        assert!(context.framebuffer().is_some());
        assert!(context.surface().is_some());
        context.swap_buffers().expect("Couldn't swap buffers.");
    }
}
