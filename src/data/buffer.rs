use crate::Context;

use glow::HasContext;

type BufferResource = <glow::Context as HasContext>::Buffer;

pub struct Buffer<'context> {
    context  : &'context Context,
    resource : BufferResource
}

impl<'context> Buffer<'context> {
    fn new(context:&'context Context) -> Buffer<'context> {
        let gl = &context.gl;
        let mut resource = unsafe {
            gl.create_buffer().expect("Couldn't create Buffer")
        };
        Buffer {context,resource}
    }

    pub fn resource(&self) -> BufferResource {
        self.resource
    }

    pub fn from_data<T>(context:&'context Context, data: &[T]) -> Buffer<'context> {
        let mut buffer = Buffer::new(context);
        buffer.set_data(data);
        buffer
    }

    pub fn allocate(context:&'context Context, size:usize) -> Buffer<'context> {
        let mut buffer = Buffer::new(context);
        if size > 0 { buffer.reallocate(size); }
        buffer
    }

    fn bind(&self) {
        let gl = &self.context.gl;
        let resource = self.resource();
        let resource = if resource == Default::default() { None } else { Some(resource) };
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, resource);
        }
    }

    pub fn get_size(&self) -> usize {
        let gl = &self.context.gl;
        self.bind();
        unsafe {
            let mut size : i32 = 0;
            //gl::GetBufferParameteriv(gl::ARRAY_BUFFER, gl::BUFFER_SIZE, &mut size as *mut i32);
            size as usize
        }
    }

    pub fn set_data<T>(&mut self, data: &[T]) {
        let gl = &self.context.gl;
        self.bind();
        unsafe {
            let slice = as_u8_slice(data.as_ref());
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, slice, glow::STATIC_DRAW);
        }
    }

    pub fn get_data<T>(&self) -> Vec<T> {
        let gl = &self.context.gl;
        self.bind();

        let size = self.get_size();
        let capacity = size / std::mem::size_of::<T>();
        let mut data : Vec<T> = Vec::with_capacity(capacity);
        unsafe {
            data.set_len(capacity);
            let offset = 0;
            let data   = as_u8_mut_slice(data.as_mut());
            gl.get_buffer_sub_data(glow::ARRAY_BUFFER, offset, data);
        }
        data
    }

    pub fn reallocate(&mut self, size: usize) {
        let gl = &self.context.gl;
        self.bind();
        unsafe {
            gl.buffer_data_size(glow::ARRAY_BUFFER, size as i32, glow::STATIC_DRAW);
        }
    }
}

impl Drop for Buffer<'_> {
    fn drop(&mut self) {
        unsafe {
            self.context.gl.delete_buffer(self.resource());
        }
    }
}



// =============
// === Utils ===
// =============

fn as_u8_mut_slice<T>(data:&mut[T]) -> &mut[u8] {
    unsafe {
        let ptr = data.as_mut_ptr();
        let len = data.len() * std::mem::size_of::<T>();
        std::slice::from_raw_parts_mut(ptr as *mut u8, len)
    }
}

fn as_u8_slice<T>(data:&[T]) -> &[u8] {
    unsafe {
        let ptr = data.as_ptr();
        let len = data.len() * std::mem::size_of::<T>();
        std::slice::from_raw_parts(ptr as *const u8, len)
    }
}