use crate::Context;

use crate::data::as_u8_slice;
use crate::data::as_u8_mut_slice;

use glow::HasContext;

use crate::TextureFormat;

type TextureResource = <glow::Context as HasContext>::Texture;

pub struct Texture<'context> {
    pub(crate) context : &'context Context,
    resource           : TextureResource,
    pub(crate) format  : TextureFormat,
    typ                : u32
}

impl<'context> Texture<'context> {
    pub fn new(context:&'context Context, format:TextureFormat, typ:u32) -> Self {
        let resource = unsafe {
            context.gl.create_texture().expect("Couldn't create texture")
        };
        Self {context,resource,format,typ}
    }

    pub fn typ(&self) -> u32 {
        self.typ
    }

    pub fn format(&self) -> &TextureFormat {
        &self.format
    }

    pub(crate) fn bind(&self) {
        let gl = &self.context.gl;
        unsafe {
            gl.bind_texture(self.typ(), Some(self.resource()));
        }
    }

    pub fn resource(&self) -> TextureResource {
        self.resource
    }
}

impl Drop for Texture<'_> {
    fn drop(&mut self) {
        unsafe {
            self.context.gl.delete_texture(self.resource());
        }
    }
}