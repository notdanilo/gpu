mod utils;

//FIXME: Test all formats in TextureFormat!

#[cfg(test)]
mod texture_3d {
    use super::utils::test;

    use gpu::ContextDisplay;
    use gpu::ContextBuilder;
    use gpu::Texture3D;
    use gpu::TextureFormat;
    use gpu::ColorFormat;
    use gpu::ComponentFormat;

    #[test]
    fn allocation() {
        panic!("UA: {}", web_sys::window().unwrap().navigator().user_agent().unwrap());
        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let context = context_builder.build();

        context.make_current().unwrap();

        let dimensions = (111, 222, 333);
        let texture = Texture3D::allocate(&context, dimensions, &TextureFormat(ColorFormat::RGBA,
                                                                               ComponentFormat::U8));
        assert_eq!(texture.dimensions(), dimensions);
    }

    #[test]
    fn from_data() {
        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let context = context_builder.build();

        context.make_current().unwrap();

        let mut data_in = Vec::new();
        let dimensions = (8, 8, 8);
        let components = 2;
        for x in 0..dimensions.0 {
            for y in 0..dimensions.1 {
                for z in 0..dimensions.2 {
                    for w in 1..(components+1) {
                        data_in.push((w * (dimensions.1 * (y * dimensions.0 + x) + z)) as u8);
                    }
                }
            }
        }

        let data_in_format = TextureFormat(ColorFormat::components(components),
                                           ComponentFormat::U8);
        let texture = Texture3D::from_data(&context, dimensions, &data_in_format, &data_in,
                                           &data_in_format);

        assert_eq!(components, texture.format().get_color_format().get_size());
        assert_eq!(dimensions, texture.dimensions());

        let data_out = texture.get_data();

        assert_eq!(data_in, data_out);
    }
}
