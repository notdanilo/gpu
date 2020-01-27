mod utils;

#[cfg(test)]
mod clear_program {
    use super::utils::test;
    use gpu::ContextBuilder;
    use gpu::ContextDisplay;
    use gpu::ClearProgram;
    use gpu::Framebuffer;

    #[test]
    fn clear_display() {
        let dimension = (320, 240);

        let context_builder = ContextBuilder::new().with_display(ContextDisplay::Window
            (String::from("clear_display (green)"), dimension.0, dimension.1));
        let context = context_builder.build();

        context.make_current().expect("Couldn't make current");

        let mut framebuffer = Framebuffer::default(&context);

        let mut clear_program = ClearProgram::new(&context);
        clear_program.set_color((0.0, 1.0, 0.0, 1.0));
        clear_program.clear(&mut framebuffer, ClearProgram::COLOR);

        context.swap_buffers().expect("Couldn't swap buffers");
    }
}