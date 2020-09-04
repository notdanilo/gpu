use crate::Context;
use crate::TextureFormat;

use glow::HasContext;

type TextureResource = <glow::Context as HasContext>::Texture;

/// A `Texture` representation.
pub struct Texture<'context> {
    pub(crate) context : &'context Context,
    resource           : TextureResource,
    pub(crate) format  : TextureFormat,
    typ                : u32
}

impl<'context> Texture<'context> {
    /// Creates a new `Texture` with the specified `TextureFormat` and the internal OpenGL `typ`.
    pub fn new(context:&'context Context, format:TextureFormat, typ:u32) -> Self {
        let resource = unsafe {
            context.gl.create_texture().expect("Couldn't create texture")
        };
        Self {context,resource,format,typ}
    }

    /// Gets the internal OpenGL type.
    pub fn typ(&self) -> u32 {
        self.typ
    }

    /// Gets the `TextureFormat`.
    pub fn format(&self) -> &TextureFormat {
        &self.format
    }

    pub(crate) fn bind(&self) {
        let gl = &self.context.gl;
        unsafe {
            gl.bind_texture(self.typ(), Some(self.resource()));
        }
    }

    /// Gets `TextureResource`.
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