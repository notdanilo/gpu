use crate::data::Buffer;
use crate::{Context, GLContext};

type VertexArrayObjectResource = u32;

// TODO: Better naming?
/// `VertexArrayObject` representation.
pub struct VertexArrayObject {
    _gl: GLContext,
    resource : VertexArrayObjectResource,
    _vertices: usize
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
        let _vertices = 0;
        Self { _gl: gl, resource, _vertices }
    }

    pub(crate) fn resource(&self) -> VertexArrayObjectResource {
        self.resource
    }

    pub(crate) fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.resource());
        }
    }

    // TODO: Allow to define the VertexBuffer component type. (It's hardcoded to gl::FALSE)
    /// Sets a `Buffer` as a vertices source, where each vertex has `n_elements`
    pub fn set_vertex_buffer(&mut self, buffer : &Buffer, attribute_index: usize, n_elements: usize) {
        self.bind();
        buffer.bind();
        unsafe {
            gl::EnableVertexAttribArray(attribute_index as u32);
            gl::VertexAttribPointer(attribute_index as u32, n_elements as i32, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        }
    }

    /// Sets a `Buffer` as a indices source.
    pub fn set_index_buffer(&mut self, buffer : &Buffer) {
        self.bind();
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.resource());
        }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.resource());
        }
    }
}
