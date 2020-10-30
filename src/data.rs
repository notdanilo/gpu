mod buffer;
mod textures;
mod vertex_array_object;
mod renderbuffer;
mod framebuffer;

pub use buffer::Buffer;
pub use textures::*;
pub use vertex_array_object::VertexArrayObject;
pub use renderbuffer::Renderbuffer;
pub use framebuffer::Framebuffer;



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