use crate::prelude::*;
use crate::Context;

use glow::HasContext;

use crate::Program;
use crate::FragmentShader;
use crate::VertexShader;
use crate::VertexArrayObject;
use crate::Framebuffer;

/// A program for rasterizing `VertexArrayObject`s in a target `Framebuffer`.
#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct RasterProgram {
    /// Program base object.
    #[shrinkwrap(main_field)]
    pub program : Program,
}

/// Kinds of raster geometries.
pub enum RasterGeometry {
    /// Raster three consecutive vertices as a triangle.
    Triangles = glow::TRIANGLES as isize,
    /// Raster each vertex as a point.
    Points    = glow::POINTS as isize
}

impl RasterProgram {
    /// Creates a new `RasterProgram` with a `FragmentShader` and ` VertexShader`.
    pub fn new(context:&Context, fragment_shader:&FragmentShader, vertex_shader:&VertexShader) -> Result<Self, String> {
        let program = Program::new(context);
        let gl      = &context.data.borrow().gl;
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
        self.context.upgrade().map(|context| {
            unsafe {
                context.data.borrow().gl.use_program(Some(self.resource()));
            }
        });
    }

    /// Draws the `n_vertices` in a `VertexArrayObject` as the specified `RasterGeometry` on the target `Framebuffer`.
    pub fn raster(&self, framebuffer: &Framebuffer, vertex_array_object: &VertexArrayObject, raster_geometry: RasterGeometry, n_vertices: u32) {
        self.context.upgrade().map(|context| {
            let gl = &context.data.borrow().gl;
            unsafe {
                framebuffer.bind();
                self.use_();
                vertex_array_object.bind();
                gl.enable(glow::PROGRAM_POINT_SIZE);
                let (width,height) = framebuffer.dimensions();
                gl.viewport(0, 0, width as i32, height as i32);
                gl.draw_arrays(raster_geometry as u32, 0, n_vertices as i32);
            }
        });
    }
}