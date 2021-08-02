use crate::{Context, GLContext};
use crate::data::as_u8_slice;

type BufferResource = u32;

/// A `Buffer` representation.
pub struct Buffer {
    _gl: GLContext,
    resource : BufferResource
}

impl Buffer {
    fn new(context:&Context) -> Self {
        // let gl = context.gl_context();
        // let resource = unsafe {
        //     let mut resource = 0;
        //     gl::CreateBuffers(1, &mut resource);
        //     resource
        // };
        // Self { _gl: gl, resource }
        unimplemented!()
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
        unimplemented!()
        // unsafe {
        //     gl::BindBuffer(gl::ARRAY_BUFFER, self.resource());
        // }
    }

    /// Gets the size in bytes.
    pub fn size(&self) -> usize {
        // self.bind();
        // unsafe {
        //     let mut params = 0;
        //     gl::GetBufferParameteriv(gl::ARRAY_BUFFER, gl::BUFFER_SIZE, &mut params);
        //     params as usize
        // }
        unimplemented!()
    }

    /// Sets the data on the GPU side.
    pub fn set_data<T>(&mut self, data: &[T]) {
        // self.bind();
        // unsafe {
        //     let slice = as_u8_slice(data);
        //     gl::BufferData(gl::ARRAY_BUFFER, slice.len() as isize, slice.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
        // }
        unimplemented!()
    }

    /// Gets the data on the GPU side.
    pub fn data<T>(&self) -> Vec<T> {
        // let size = self.size();
        // let capacity = size / std::mem::size_of::<T>();
        // let mut data : Vec<T> = Vec::with_capacity(capacity);
        // self.bind();
        // unsafe {
        //     data.set_len(capacity);
        //     let offset = 0;
        //     gl::GetBufferSubData(gl::ARRAY_BUFFER, offset, size as isize, data.as_ptr() as *mut std::ffi::c_void);
        // }
        // data
        unimplemented!()
    }

    /// Reallocates the memory with `size`.
    pub fn reallocate(&mut self, size: usize) {
        // self.bind();
        // unsafe {
        //     gl::BufferData(gl::ARRAY_BUFFER, size as isize, std::ptr::null(), gl::STATIC_DRAW);
        // }
        unimplemented!()
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        // unsafe {
        //     gl::DeleteBuffers(1, &self.resource());
        // }
        unimplemented!()
    }
}