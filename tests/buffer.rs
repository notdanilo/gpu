mod utils;

#[cfg(test)]
mod buffer {
    use super::utils::test;
    // use gpu::ContextBuilder;
    // use gpu::ContextDisplay;
    // use gpu::Buffer;


    #[test]
    fn allocation() {
        // let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        // let context = context_builder.build();
        //
        // context.make_current().unwrap();
        //
        // let buffer = Buffer::allocate(&context,12345);
        // assert_eq!(buffer.size(), 12345);
        unimplemented!()
    }

    #[test]
    fn from_data() {
        // let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        // let context = context_builder.build();
        //
        // context.make_current().unwrap();
        //
        // let mut data_in = Vec::new();
        // for i in 0..10 { data_in.push(i as f32); }
        //
        // let buffer = Buffer::from_data(&context,&data_in);
        // let data_out = buffer.data();
        //
        // assert_eq!(data_in, data_out);
        unimplemented!()
    }
}