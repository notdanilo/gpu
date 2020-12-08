mod image_2d;
mod image_3d;
mod sampler;

mod color_format;
mod r#type;
mod image_format;

pub use sampler::*;

pub use color_format::ColorFormat;
pub use r#type::Type;
pub use image_format::ImageFormat;

pub use image_2d::Image2D;
pub use image_3d::Image3D;

use crate::{Context, GLContext};
use std::rc::Rc;

struct ImageResource {
    resource: u32
}

impl Drop for ImageResource {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.resource);
        }
    }
}

/// A `Image` representation.
#[derive(Clone)]
pub struct Image {
    pub(crate) _gl : GLContext,
    resource       : Rc<ImageResource>,
    format         : ImageFormat,
    type_          : u32
}

impl Image {
    /// Creates a new `Image` with the specified `ImageFormat` and the internal OpenGL `Type`.
    pub fn new(context:&Context, format: &ImageFormat, type_:u32) -> Self {
        let format = format.clone();
        let gl = context.gl_context();
        let resource = unsafe {
            let mut resource = 0;
            gl::GenTextures(1, &mut resource);
            resource
        };
        let resource = Rc::new(ImageResource { resource });
        Self { _gl: gl, resource, format, type_ }
    }

    /// Gets the internal OpenGL type.
    pub fn type_(&self) -> u32 {
        self.type_
    }

    /// Gets the `TextureFormat`.
    pub fn format(&self) -> &ImageFormat {
        &self.format
    }

    pub(crate) fn bind(&self) {
        unsafe {
            gl::BindTexture(self.type_(), self.internal());
        }
    }

    /// Gets `TextureResource`.
    pub(crate) fn internal(&self) -> u32 {
        self.resource.resource
    }
}