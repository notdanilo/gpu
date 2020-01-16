use crate::Resource;
use crate::Context;
use glow::HasContext;

pub struct Renderbuffer<'context> {
    context : &'context Context,
    id      : u32
}

impl<'context> Renderbuffer<'context> {
    pub fn default(context:&'context Context) -> Self {
        let id = 0;
        Self {id,context}
    }

    pub fn new(context:&'context Context, width: u32, height: u32) -> Self {
        let gl     = &context.gl;
        let mut id = 0;
        let width  = width as i32;
        let height = height as i32;
        unsafe {
            id = gl.create_renderbuffer().expect("Couldn't create Renderbuffer");
            gl.bind_renderbuffer(glow::RENDERBUFFER, Some(id));
            gl.renderbuffer_storage(glow::RENDERBUFFER, glow::DEPTH_COMPONENT, width, height);
        }

        Self {context,id}
    }
}

impl<'context> Drop for Renderbuffer<'context> {
    fn drop(&mut self) {
        unsafe {
            self.context.gl.delete_renderbuffer(self.get_id());
        }
    }
}

impl<'context> Resource for Renderbuffer<'context> {
    fn get_id(&self) -> u32 { self.id }
}
