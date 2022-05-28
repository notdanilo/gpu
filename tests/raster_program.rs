mod utils;

#[cfg(test)]
mod clear_program {
    use super::utils::test;

    use gpu::{ContextBuilder, Window};
    use gpu::ContextDisplay;
    use gpu::Framebuffer;
    use gpu::VertexShader;
    use gpu::FragmentShader;
    use gpu::RasterProgram;
    use gpu::RasterGeometry;
    use gpu::Buffer;
    use gpu::VertexArrayObject;



    #[test]
    fn draw_to_display() {
        let components = 3;
        let dimension = (320, 240);
        let window = Window::new("draw_to_display (red)".into(), dimension);
        let display = ContextDisplay::Window(window);
        let context_builder = ContextBuilder::new().with_display(display);
        let context = context_builder.build();

        context.make_current().unwrap();

        let vertex_shader = VertexShader::new(&context, r#"#version 300 es
            layout(location = 0) in vec3 position;

            void main() {
                gl_Position = vec4(position, 1.0);
                gl_PointSize = 320.0;
            }
        "#).unwrap();

        let fragment_shader = FragmentShader::new(&context, r#"#version 300 es
            precision highp float;
            layout(location = 0) out vec4 color;
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#).unwrap();

        let raster_program = RasterProgram::new(&context, &vertex_shader, &fragment_shader)
            .unwrap();

        let framebuffer = Framebuffer::default(&context);

        let mut expected_data : Vec<u8> = Vec::new();
        for _x in 0..dimension.0 {
            for _y in 0..dimension.1 {
                expected_data.push(255);
                expected_data.push(0);
                expected_data.push(0);
            }
        }

        let buffer_data = vec![0.0, 0.0, 0.0];
        let buffer = Buffer::from_data(&context, &buffer_data);
        let mut vao = VertexArrayObject::new(&context);
        vao.set_vertex_buffer(&buffer, 0, 3);

        raster_program.raster(&framebuffer, &vao, RasterGeometry::Points, 1);

        let capacity = dimension.0 * dimension.1 * components;
        let mut data_out : Vec<u8> = Vec::with_capacity(capacity);
        unsafe {
            data_out.set_len(capacity);
        }

        // Wrap this functionality somewhere in the API?
//        unsafe {
//            gl::NamedFramebufferReadBuffer(0, gl::BACK);
//            gl::ReadPixels(0, 0, dimension.0 as i32, dimension.1 as i32, gl::RGB, gl::UNSIGNED_BYTE, data_out.as_mut_ptr() as *mut c_void);
//            assert_eq!(gl::GetError(), 0);
//        }
//        assert_eq!(expected_data, data_out);

        context.swap_buffers().unwrap();

//        thread::sleep(time::Duration::from_millis(1000));
//
//        unsafe {
//            gl::NamedFramebufferReadBuffer(0, gl::FRONT);
//            gl::ReadPixels(0, 0, dimension.0 as i32, dimension.1 as i32, gl::RGB, gl::UNSIGNED_BYTE, data_out.as_mut_ptr() as *mut c_void);
//            assert_eq!(gl::GetError(), 0);
//        }
//        assert_eq!(expected_data, data_out);
    }

    #[test]
    fn draw_to_image2d() {
        use gpu::{Image2D, ColorFormat, ImageFormat, Type, Buffer};

        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let context = context_builder.build();

        context.make_current().unwrap();

        let vertex_shader = VertexShader::new(&context, r#"#version 300 es
            layout(location = 0) in vec3 position;

            void main() {
                gl_Position = vec4(position, 1.0);
                gl_PointSize = 8.0;
            }
        "#).unwrap();

        let fragment_shader = FragmentShader::new(&context, r#"#version 300 es
            precision highp float;
            layout(location = 0) out vec4 color;
            void main() {
                color = vec4(1.0, 2.0, 3.0, 4.0);
            }
        "#).unwrap();

        let raster_program = RasterProgram::new(&context, &vertex_shader, &fragment_shader)
            .unwrap();

        let components = 4;
        let format = ImageFormat::new(ColorFormat::components(components), Type::F32);
        let dimension = (8, 8);
        let color = Image2D::allocate(&context, dimension, &format);
        let framebuffer = Framebuffer::new(&context, Some(color), None, None).unwrap();

        let mut expected_data : Vec<f32> = Vec::new();
        for _x in 0..dimension.0 {
            for _y in 0..dimension.1 {
                for c in 1..(components+1) {
                    expected_data.push(c as f32);
                }
            }
        }

        let buffer_data = vec![0.0, 0.0, 0.0];
        let buffer = Buffer::from_data(&context, &buffer_data);
        let mut vao = VertexArrayObject::new(&context);
        vao.set_vertex_buffer(&buffer, 0, 3);

        raster_program.raster(&framebuffer, &vao, RasterGeometry::Points, 1);
        let data_out : Vec<f32> = framebuffer.color().unwrap().data();

       assert_eq!(expected_data, data_out);
    }
}