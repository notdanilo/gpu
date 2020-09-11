use crate::prelude::*;
use crate::{Context, GLContext};

type BufferResource = <glow::Context as HasContext>::Buffer;

use super::as_u8_mut_slice;
use super::as_u8_slice;

/// A `Buffer` representation.
pub struct Buffer {
    gl       : GLContext,
    resource : BufferResource
}

impl Buffer {
    fn new(context:&Context) -> Self {
        let gl = context.gl_context();
        let resource = unsafe {
            gl.create_buffer().expect("Couldn't create Buffer")
        };
        Self { gl, resource }
    }

    /// Gets the `BufferResource`.
    pub fn resource(&self) -> BufferResource {
        self.resource
    }

    /// Creates a new `Buffer` from a slice.
    pub fn from_data<T>(context:&Context, data: &[T]) -> Self {
        let mut buffer = Buffer::new(context);
        buffer.set_data(data);
        buffer
    }

    /// Allocates a new `Buffer` with `n_bytes`.
    pub fn allocate(context:&Context, n_bytes:usize) -> Self {
        let mut buffer = Buffer::new(context);
        if n_bytes > 0 { buffer.reallocate(n_bytes); }
        buffer
    }

    pub(crate) fn bind(&self) {
        let resource = self.resource();
        let resource = if resource == Default::default() { None } else { Some(resource) };
        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, resource);
        }
    }

    /// Gets the size in bytes.
    pub fn size(&self) -> usize {
        self.bind();
        unsafe {
            self.gl.get_buffer_parameter_i32(glow::ARRAY_BUFFER, glow::BUFFER_SIZE) as usize
        }
    }

    /// Sets the data on the GPU side.
    pub fn set_data<T>(&mut self, data: &[T]) {
        self.bind();
        unsafe {
            let slice = as_u8_slice(data);
            self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, slice, glow::STATIC_DRAW);
        }
    }

    /// Gets the data on the GPU side.
    pub fn data<T>(&self) -> Vec<T> {
        let size = self.size();
        let capacity = size / std::mem::size_of::<T>();
        let mut data : Vec<T> = Vec::with_capacity(capacity);
        self.bind();
        unsafe {
            data.set_len(capacity);
            let offset = 0;
            let data   = as_u8_mut_slice(data.as_mut());
            self.gl.get_buffer_sub_data(glow::ARRAY_BUFFER, offset, data);
        }
        data
    }

    /// Reallocates the memory with `size`.
    pub fn reallocate(&mut self, size: usize) {
        self.bind();
        unsafe {
            self.gl.buffer_data_size(glow::ARRAY_BUFFER, size as i32, glow::STATIC_DRAW);
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.resource());
        }
    }
}