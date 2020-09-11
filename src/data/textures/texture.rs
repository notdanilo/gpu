use crate::prelude::*;
use crate::{Context, GLContext};
use crate::TextureFormat;

type TextureResource = <glow::Context as HasContext>::Texture;

/// A `Texture` representation.
pub struct Texture {
    pub(crate) gl      : GLContext,
    resource           : TextureResource,
    pub(crate) format  : TextureFormat,
    typ                : u32
}

impl Texture {
    /// Creates a new `Texture` with the specified `TextureFormat` and the internal OpenGL `typ`.
    pub fn new(context:&Context, format:TextureFormat, typ:u32) -> Self {
        let gl = context.gl_context();
        let resource = unsafe {
            gl.create_texture().expect("Couldn't create texture")
        };
        Self { gl, resource, format, typ }
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
        unsafe {
            self.gl.bind_texture(self.typ(), Some(self.resource()));
        }
    }

    /// Gets `TextureResource`.
    pub fn resource(&self) -> TextureResource {
        self.resource
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.resource());
        }
    }
}