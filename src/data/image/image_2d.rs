use crate::prelude::*;
use crate::Context;

use crate::ImageFormat;
use crate::Image;


/// An `Image2D` representation.
#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Image2D {
    /// Base texture object.
    #[shrinkwrap(main_field)]
    pub image: Image,
    dimensions : (usize,usize)
}

impl Image2D {
    fn new(context:&Context, format: &ImageFormat) -> Self {
        let image      = Image::new(context, format, gl::TEXTURE_2D);
        let dimensions = (0,0);
        Self { image,dimensions}
    }

    /// Gets the dimensions.
    pub fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    /// Allocates a new `Image2D` with the specified dimensions and `TextureFormat`.
    pub fn allocate
    (context:&Context, dimensions:(usize, usize), format:&ImageFormat) -> Self {
        let mut texture = Self::new(context, format);
        texture.reallocate(dimensions, &format);
        texture
    }

    /// Creates a new `Image2D` from a slice.
    pub fn from_data<T>
    (context:&Context
     , dimensions:(usize,usize)
     , format:&ImageFormat
     , data: &[T]
     , data_format:&ImageFormat) -> Self {
        let mut texture = Self::allocate(context, dimensions, format);
        texture.set_data(dimensions, &format, data, &data_format);
        texture
    }

    /// Reallocates the memory on the GPU side.
    pub fn reallocate(&mut self, dimensions: (usize, usize), format: &ImageFormat) {
        self.dimensions = dimensions;
        self.format     = format.clone();
        self.bind();
        unsafe {
            let tex_type        = self.type_();
            let internal_format = format.internal_format();
            gl::TexStorage2D(tex_type, 1, internal_format, dimensions.0 as i32, dimensions.1 as i32);
        }
    }

    /// Gets a copy of the data on the GPU.
    pub fn set_data<T>(&mut self, dimensions: (usize, usize), format: &ImageFormat, data: &[T], data_format: &ImageFormat) {
        self.dimensions = dimensions;
        self.format     = format.clone();
        unsafe {
            let (color, ty) = data_format.get_format_and_type();
            let width= dimensions.0 as i32;
            let height= dimensions.1 as i32;
            gl::TextureSubImage2D(self.internal(), 0, 0, 0, width, height, color, ty, data.as_ptr() as *const std::ffi::c_void);
        }
    }

    /// Gets a copy of the data on the GPU.
    pub fn data<T>(&self) -> Vec<T> {
        let (width,height)    = self.dimensions();
        let capacity          = width * height * self.format().color_format().size();
        let mut data : Vec<T> = Vec::with_capacity(capacity);
        unsafe {
            data.set_len(capacity);
            let (format, type_) = self.image.format().get_format_and_type();

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.internal());
            // TODO: Use glGetTextureSubImage here.
            gl::GetTexImage(gl::TEXTURE_2D, 0, format, type_, data.as_mut_ptr() as *mut std::ffi::c_void);
        }
        data
    }
}