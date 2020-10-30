use crate::{Context, GLContext};
use crate::TextureFormat;

type TextureResource = u32;

/// A `Texture` representation.
pub struct Texture {
    pub(crate) gl      : GLContext,
    resource           : TextureResource,
    pub(crate) format  : TextureFormat,
    type_              : u32
}

impl Texture {
    /// Creates a new `Texture` with the specified `TextureFormat` and the internal OpenGL `typ`.
    pub fn new(context:&Context, format:TextureFormat, type_:u32) -> Self {
        let gl = context.gl_context();
        let resource = unsafe {
            let mut resource = 0;
            gl::GenTextures(1, &mut resource);
            resource
        };
        Self { gl, resource, format, type_ }
    }

    /// Gets the internal OpenGL type.
    pub fn type_(&self) -> u32 {
        self.type_
    }

    /// Gets the `TextureFormat`.
    pub fn format(&self) -> &TextureFormat {
        &self.format
    }

    pub(crate) fn bind(&self) {
        unsafe {
            gl::BindTexture(self.type_(), self.resource());
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
            gl::DeleteTextures(1, &self.resource());
        }
    }
}