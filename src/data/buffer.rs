use std::ffi::c_void;
use std::ptr;

use crate::Resource;

pub struct Buffer {
    id : u32
}

impl Resource for Buffer {
    fn get_id(&self) -> u32 { self.id }
}

impl Buffer {
    fn new() -> Buffer {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Buffer {
            id
        }
    }

    pub fn from_data<T>(data: &[T]) -> Buffer {
        let mut buffer = Buffer::new();
        buffer.set_data(data);
        buffer
    }

    pub fn allocate(size: usize) -> Buffer {
        let mut buffer = Buffer::new();
        if size > 0 { buffer.reallocate(size); }
        buffer
    }

    pub fn get_size(&self) -> usize {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            let mut size : i32 = 0;
            gl::GetBufferParameteriv(gl::ARRAY_BUFFER, gl::BUFFER_SIZE, &mut size as *mut i32);
            size as usize
        }
    }

    pub fn set_data<T>(&mut self, data: &[T]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<T>() * data.len()) as isize, &data[0] as *const T as *const c_void, gl::STATIC_DRAW);
        }
    }

    pub fn get_data<T>(&self) -> Vec<T> {
        let size = self.get_size();
        let capacity = size / std::mem::size_of::<T>();
        let mut data : Vec<T> = Vec::with_capacity(capacity);
        unsafe {
            data.set_len(capacity);
            let offset = 0;
            gl::GetBufferSubData(gl::ARRAY_BUFFER, offset, size as isize, data.as_mut_ptr() as *mut c_void);
        }
        data
    }

    pub fn reallocate(&mut self, size: usize) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferData(gl::ARRAY_BUFFER, size as isize, ptr::null(), gl::STATIC_DRAW);
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.get_id());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ContextBuilder, ContextDisplay, Buffer, initialize};
    #[test]
    fn allocation() {
        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let mut context = context_builder.build();

        context.make_current().unwrap();
        initialize(|symbol| context.get_proc_address(symbol) as *const _);

        let buffer = Buffer::allocate(12345);
        assert_eq!(buffer.get_size(), 12345);
    }

    #[test]
    fn from_data() {
        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let mut context = context_builder.build();

        context.make_current().unwrap();
        initialize(|symbol| context.get_proc_address(symbol) as *const _);

        let mut data_in = Vec::new();
        for i in 0..10 { data_in.push(i as f32); }

        let buffer = Buffer::from_data(&data_in);
        let data_out = buffer.get_data();

        assert_eq!(data_in, data_out);
    }
}
