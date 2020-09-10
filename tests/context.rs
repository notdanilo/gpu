mod utils;

#[cfg(test)]
mod context {
    use super::utils::test;
    use gpu::ContextBuilder;
    use gpu::ContextDisplay;
    use gpu::prelude::*;

    #[test]
    fn create_context() {
        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let context = context_builder.build();

        context.make_current().expect("Couldn't make current");
    }

    #[test]
    fn present_context() {
        let context_builder = ContextBuilder::new().with_display(ContextDisplay::Window(String::from("present_context (black)"), 320, 240));
        let context = context_builder.build();

        context.make_current().expect("Couldn't make current");

        context.swap_buffers().expect("Couldn't swap buffers");
    }
}