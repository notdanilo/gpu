mod utils;

//FIXME: Test all formats in TextureFormat!

#[cfg(test)]
mod texture_2d {
    use super::utils::test;

    use gpu::ContextDisplay;
    use gpu::ContextBuilder;
    use gpu::Image2D;
    use gpu::ImageFormat;
    use gpu::ColorFormat;
    use gpu::Type;



    #[test]
    fn allocation() {
        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let context = context_builder.build();

        context.make_current().unwrap();

        let dimension = (123, 321);
        let texture = Image2D::allocate(&context, dimension, &ImageFormat(ColorFormat::RGBA, Type::U8));
        assert_eq!(texture.dimensions(), dimension);
    }

    #[test]
    fn from_data() {
        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let context = context_builder.build();

        context.make_current().unwrap();

        let mut data_in = Vec::new();
        let dimension = (8, 8);
        let components = 2;
        for x in 0..dimension.0 {
            for y in 0..dimension.1 {
                for z in 1..(components + 1) {
                    data_in.push((z * (y * dimension.0 + x)) as u8);
                }
            }
        }

        let data_in_format = ImageFormat(ColorFormat::components(components),
                                         Type::U8);
        let texture = Image2D::from_data(&context, dimension, &data_in_format, &data_in,
                                         &data_in_format);

        assert_eq!(components, texture.format().color_format().size());
        assert_eq!(dimension, texture.dimensions());

        let data_out = texture.data();

        assert_eq!(data_in, data_out);
    }
}