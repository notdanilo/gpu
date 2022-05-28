mod utils;

#[cfg(test)]
mod context {
    use super::utils::test;
    use gpu::{ContextBuilder, Window};
    use gpu::ContextDisplay;


    #[test]
    fn create_context() {
        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let context = context_builder.build();

        context.make_current().expect("Couldn't make current");
    }

    #[test]
    fn present_context() {
        let window = Window::new("present_context (black)".into(), (320, 240));
        let display = ContextDisplay::Window(window);
        let context_builder = ContextBuilder::new().with_display(display);
        let context = context_builder.build();

        context.make_current().expect("Couldn't make current");

        context.swap_buffers().expect("Couldn't swap buffers");
    }
}