mod utils;

//FIXME: Test all formats in TextureFormat!

#[cfg(test)]
mod texture_3d {
    use super::utils::test;

    use gpu::ContextDisplay;
    use gpu::ContextBuilder;
    use gpu::Image3D;
    use gpu::ImageFormat;
    use gpu::ColorFormat;
    use gpu::Type;



    #[test]
    fn allocation() {
        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let context = context_builder.build();

        context.make_current().unwrap();

        let dimensions = (111, 222, 333);
        let texture = Image3D::allocate(&context, dimensions, &ImageFormat(ColorFormat::RGBA,
                                                                           Type::U8));
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

        let data_in_format = ImageFormat(ColorFormat::components(components),
                                         Type::U8);
        let texture = Image3D::from_data(&context, dimensions, &data_in_format, &data_in,
                                         &data_in_format);

        assert_eq!(components, texture.format().color_format().size());
        assert_eq!(dimensions, texture.dimensions());

        let data_out = texture.data();

        assert_eq!(data_in, data_out);
    }
}
