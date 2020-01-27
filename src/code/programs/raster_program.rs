use crate::Context;

use glow::HasContext;

use crate::Program;
use crate::FragmentShader;
use crate::VertexShader;
use crate::VertexArrayObject;
use crate::Framebuffer;

use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct RasterProgram<'context> {
    #[shrinkwrap(main_field)]
    pub program : Program<'context>,
}

pub enum RasterGeometry {
    Triangles = glow::TRIANGLES as isize,
    Points    = glow::POINTS as isize
}

impl<'context> RasterProgram<'context> {
    pub fn new(context:&'context Context, fragment_shader:&FragmentShader, vertex_shader:&VertexShader) -> Result<Self, String> {
        let program = Program::new(context);
        let gl      = &context.gl;
        unsafe {
            gl.attach_shader(program.resource(), vertex_shader.resource());
            gl.attach_shader(program.resource(), fragment_shader.resource());
            gl.link_program(program.resource());

            // Check for linking errors
            if !gl.get_program_link_status(program.resource()) {
                return Err(gl.get_program_info_log(program.resource()))
            }
        }

        Ok(Self {program})
    }

    pub(crate) fn use_(&self) {
        unsafe {
            self.context.gl.use_program(Some(self.resource()));
        }
    }

    pub fn raster(&self, framebuffer: &Framebuffer, vao: &VertexArrayObject, geometry : RasterGeometry, elements: u32) {
        let gl = &self.context.gl;
        unsafe {
            framebuffer.bind();
            self.use_();
            vao.bind();
            gl.enable(glow::PROGRAM_POINT_SIZE);
            let (width,height) = framebuffer.dimensions();
            gl.viewport(0, 0, width as i32, height as i32);
            gl.draw_arrays(geometry as u32, 0, elements as i32);
        }
    }
}