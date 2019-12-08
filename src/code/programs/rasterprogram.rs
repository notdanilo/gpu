use crate::code::Program;
use crate::code::FragmentShader;
use crate::code::VertexShader;
use crate::Resource;

use crate::data::Framebuffer;
use crate::data::VertexArrayObject;

use std::ptr;
use gl::types::*;

pub struct RasterProgram {
    id : u32
}

impl Program for RasterProgram {
    fn get_id(&self) -> u32 { self.id }
}

pub enum RasterGeometry {
    Triangles = gl::TRIANGLES as isize,
    Points = gl::POINTS as isize
}

impl RasterProgram {
    pub const TRIANGLES : u32 = gl::TRIANGLES;
    pub const POINTS : u32 = gl::POINTS;
    pub fn new(fragment_shader : &FragmentShader, vertex_shader : &VertexShader) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vertex_shader.get_id());
            gl::AttachShader(id, fragment_shader.get_id());
            gl::LinkProgram(id);

            // Check for linking errors
            let mut success = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                let mut len = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let mut info_log = Vec::with_capacity(len as usize);
                info_log.set_len((len as usize) - 1); // -1 to skip trialing null character

                gl::GetProgramInfoLog(id, len, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                return Err(String::from_utf8(info_log).unwrap());
            }
        }

        Ok(Self {
            id
        })
    }

    pub fn raster(&self, framebuffer: &Framebuffer, vao: &VertexArrayObject, geometry : RasterGeometry, elements: u32) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer.get_id());
            gl::UseProgram(self.get_id());
            gl::BindVertexArray(vao.get_id());
            gl::Enable(gl::PROGRAM_POINT_SIZE);
            let dimension = framebuffer.get_dimension();
            gl::Viewport(0, 0, dimension.0 as i32, dimension.1 as i32);
            gl::DrawArrays(geometry as u32, 0, elements as i32);
        }
    }
}

impl Drop for RasterProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.get_id()); }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ContextBuilder, ContextDisplay, initialize, RasterProgram, VertexShader, FragmentShader, VertexArrayObject, RasterGeometry, Framebuffer};
    #[test]
    fn draw_to_display() {
        use crate::{Buffer};
        use std::{thread, time};
        use std::ffi::c_void;

        let components = 3;
        let dimension = (320, 240);

        let context_builder = ContextBuilder::new().with_display(ContextDisplay::Window(String::from("draw_to_display (red)"), dimension.0, dimension.1));
        let mut context = context_builder.build();

        context.make_current().unwrap();
        initialize(|symbol| context.get_proc_address(symbol) as *const _);

        let vertex_shader = VertexShader::new(r#"
            #version 460 core
            in layout(location = 0) vec3 position;

            void main() {
                gl_Position = vec4(position, 1.0);
                gl_PointSize = 320.0;
            }
        "#).unwrap();

        let fragment_shader = FragmentShader::new(r#"
            #version 460 core
            void main() {
                gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#).unwrap();

        let raster_program = RasterProgram::new(&fragment_shader, &vertex_shader).unwrap();

        let framebuffer = Framebuffer::default();

        let mut expected_data : Vec<u8> = Vec::new();
        for _x in 0..dimension.0 {
            for _y in 0..dimension.1 {
                expected_data.push(255);
                expected_data.push(0);
                expected_data.push(0);
            }
        }

        let buffer_data = vec![0.0, 0.0, 0.0];
        let buffer = Buffer::from_data(&buffer_data);
        let mut vao = VertexArrayObject::new();
        vao.set_vertex_buffer(&buffer, 0, 3);

        raster_program.raster(&framebuffer, &vao, RasterGeometry::Points, 1);

        let capacity = dimension.0 * dimension.1 * components;
        let mut data_out : Vec<u8> = Vec::with_capacity(capacity);
        unsafe {
            data_out.set_len(capacity);
        }

        // Wrap this functionality somewhere in the API?
        unsafe {
            gl::NamedFramebufferReadBuffer(0, gl::BACK);
            gl::ReadPixels(0, 0, dimension.0 as i32, dimension.1 as i32, gl::RGB, gl::UNSIGNED_BYTE, data_out.as_mut_ptr() as *mut c_void);
            assert_eq!(gl::GetError(), 0);
        }
        assert_eq!(expected_data, data_out);

        context.swap_buffers().unwrap();

        thread::sleep(time::Duration::from_millis(1000));

        unsafe {
            gl::NamedFramebufferReadBuffer(0, gl::FRONT);
            gl::ReadPixels(0, 0, dimension.0 as i32, dimension.1 as i32, gl::RGB, gl::UNSIGNED_BYTE, data_out.as_mut_ptr() as *mut c_void);
            assert_eq!(gl::GetError(), 0);
        }
        assert_eq!(expected_data, data_out);
    }

    #[test]
    fn draw_to_texture2d() {
        use crate::{Texture2D, ColorFormat, TextureFormat, ComponentFormat, Buffer};

        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let mut context = context_builder.build();

        context.make_current().unwrap();
        initialize(|symbol| context.get_proc_address(symbol) as *const _);

        let vertex_shader = VertexShader::new(r#"
            #version 460 core
            in layout(location = 0) vec3 position;

            void main() {
                gl_Position = vec4(position, 1.0);
                gl_PointSize = 8.0;
            }
        "#).unwrap();

        let fragment_shader = FragmentShader::new(r#"
            #version 460 core
            void main() {
                gl_FragColor = vec4(1.0, 2.0, 3.0, 4.0);
            }
        "#).unwrap();

        let raster_program = RasterProgram::new(&fragment_shader, &vertex_shader).unwrap();

        let components = 4;
        let format = TextureFormat::new(ColorFormat::components(components), ComponentFormat::F32);
        let dimension = (8, 8);
        let color = Texture2D::allocate(dimension, &format);
        let framebuffer = Framebuffer::new(Some(color), None, None).unwrap();

        let mut expected_data : Vec<f32> = Vec::new();
        for _x in 0..dimension.0 {
            for _y in 0..dimension.1 {
                for c in 1..(components+1) {
                    expected_data.push(c as f32);
                }
            }
        }

        let buffer_data = vec![0.0, 0.0, 0.0];
        let buffer = Buffer::from_data(&buffer_data);
        let mut vao = VertexArrayObject::new();
        vao.set_vertex_buffer(&buffer, 0, 3);

        raster_program.raster(&framebuffer, &vao, RasterGeometry::Points, 1);
        let data_out : Vec<f32> = framebuffer.get_color().unwrap().get_data();

        unsafe {
            assert_eq!(gl::GetError(), 0);
        }
        assert_eq!(expected_data, data_out);
    }
}
