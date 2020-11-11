use crate::data::Buffer;
use crate::{Context, GLContext};

type VertexArrayObjectResource = u32;

/// `VertexArrayObject` representation.
pub struct VertexArrayObject {
    gl       : GLContext,
    resource : VertexArrayObjectResource,
    vertices : usize
}

impl VertexArrayObject {
    /// Creates a new `VertexArrayObject`.
    pub fn new(context:&Context) -> Self {
        let gl = context.gl_context();
        let resource = unsafe {
            let mut resource = 0;
            gl::CreateVertexArrays(1, &mut resource);
            resource
        };
        let vertices = 0;
        Self { gl, resource, vertices }
    }

    pub(crate) fn resource(&self) -> VertexArrayObjectResource {
        self.resource
    }

    pub(crate) fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.resource());
        }
    }

    /// Sets a `Buffer` as a vertices sources, where each vertex has `n_elements`
    pub fn set_vertex_buffer(&mut self, buffer : &Buffer, attribute_index: usize, n_elements: usize) {
        self.bind();
        buffer.bind();
        unsafe {
            gl::EnableVertexAttribArray(attribute_index as u32);
            gl::VertexAttribPointer(attribute_index as u32, n_elements as i32, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        }
    }

    /// Sets the number of vertices.
    pub fn set_vertices(&mut self, vertices : usize) {
        self.vertices = vertices;
    }

    /// Gets the number of vertices.
    pub fn get_vertices(&self) -> usize {
        self.vertices
    }

    // TODO:
    // pub fn set_index_buffer(&mut self, buffer : &Buffer, elements: u32) {
    //     unsafe {
    //         gl::BindVertexArray(self.id);
    //         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.id);
    //         gl::
    //     }
    // }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.resource());
        }
    }
}
