use crate::Resource;
use crate::data::Buffer;

pub struct VertexArrayObject {
    id : u32,
    vertices: u32
}

impl Resource for VertexArrayObject {
    fn get_id(&self) -> u32 { self.id }
}

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        };

        VertexArrayObject {
            id : id,
            vertices : 0
        }
    }

    pub fn set_vertex_buffer(&mut self, buffer : &Buffer, index: u32, elements: u32) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, buffer.get_id());
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(index, elements as i32, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        }
    }

    pub fn set_vertices(&mut self, vertices : u32) {
        self.vertices = vertices;
    }

    pub fn get_vertices(&self) -> u32 { self.vertices }

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
            gl::DeleteVertexArrays(1, &mut self.id);
        }
    }
}
