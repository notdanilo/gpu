use crate::data::Buffer;

use crate::Context;
use glow::HasContext;

type VertexArrayObjectResource = <glow::Context as HasContext>::VertexArray;

pub struct VertexArrayObject<'context> {
    context  : &'context Context,
    resource : VertexArrayObjectResource,
    vertices : u32
}

impl<'context> VertexArrayObject<'context> {
    pub fn new(context:&'context Context) -> Self {
        let resource = unsafe {
            context.gl.create_vertex_array().expect("Couldn't create VertexArrayObject")
        };
        let vertices = 0;
        Self {context,resource,vertices}
    }

    pub(crate) fn resource(&self) -> VertexArrayObjectResource {
        self.resource
    }

    pub(crate) fn bind(&self) {
        unsafe {
            self.context.gl.bind_vertex_array(Some(self.resource()));
        }
    }

    pub fn set_vertex_buffer(&mut self, buffer : &Buffer, index: u32, elements: u32) {
        let gl = &self.context.gl;
        self.bind();
        buffer.bind();
        unsafe {
            gl.enable_vertex_attrib_array(index);
            gl.vertex_attrib_pointer_f32(index, elements as i32, glow::FLOAT, false, 0, 0);
        }
    }

    pub fn set_vertices(&mut self, vertices : u32) {
        self.vertices = vertices;
    }

    pub fn get_vertices(&self) -> u32 {
        self.vertices
    }

    // pub fn set_index_buffer(&mut self, buffer : &Buffer, elements: u32) {
    //     unsafe {
    //         gl::BindVertexArray(self.id);
    //         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.id);
    //         gl::
    //     }
    // }
}

impl Drop for VertexArrayObject<'_> {
    fn drop(&mut self) {
        unsafe {
            self.context.gl.delete_vertex_array(self.resource());
        }
    }
}
