use crate::prelude::*;
use crate::Context;

use crate::TextureFormat;
use crate::ColorFormat;
use crate::Type;
use crate::Texture;


/// A `Texture2D` representation.
#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Texture2D {
    /// Base texture object.
    #[shrinkwrap(main_field)]
    pub texture : Texture,
    dimensions : (usize,usize)
}

impl Texture2D {
    fn new(context:&Context) -> Self {
        let format     = TextureFormat::new(ColorFormat::RGBA, Type::F32);
        let texture    = Texture::new(context, format, gl::TEXTURE_2D);
        let dimensions = (0,0);
        Self {texture,dimensions}
    }

    /// Gets the dimensions.
    pub fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    /// Allocates a new `Texture2D` with the specified dimensions and `TextureFormat`.
    pub fn allocate
    (context:&Context, dimensions:(usize, usize), format:&TextureFormat) -> Self {
        let mut texture = Self::new(context);
        texture.reallocate(dimensions, &format);
        texture
    }

    /// Creates a new `Texture2D` from a slice.
    pub fn from_data<T>
    ( context:&Context
    , dimensions:(usize,usize)
    , format:&TextureFormat
    , data: &[T]
    , data_format:&TextureFormat) -> Self {
        let mut texture = Self::allocate(context, dimensions, format);
        texture.set_data(dimensions, &format, data, &data_format);
        texture
    }

    /// Reallocates the memory on the GPU side.
    pub fn reallocate(&mut self, dimensions: (usize, usize), format: &TextureFormat) {
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
    pub fn set_data<T>(&mut self, dimensions: (usize, usize), format: &TextureFormat, data: &[T], data_format: &TextureFormat) {
        self.dimensions = dimensions;
        self.format     = format.clone();
        unsafe {
            let (color, ty)     = data_format.get_format_type();
            let width           = dimensions.0 as i32;
            let height          = dimensions.1 as i32;
            gl::TextureSubImage2D(self.resource(),0,0,0,width,height,color,ty,data.as_ptr() as *const std::ffi::c_void);
        }
    }

    /// Gets a copy of the data on the GPU.
    pub fn data<T>(&self) -> Vec<T> {
        let (width,height)    = self.dimensions();
        let capacity          = width * height * self.format().color_format().size();
        let mut data : Vec<T> = Vec::with_capacity(capacity);
        unsafe {
            data.set_len(capacity);
            let (format, type_) = self.texture.format().get_format_type();

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.resource());
            // TODO: Use glGetTextureSubImage here.
            gl::GetTexImage(gl::TEXTURE_2D, 0, format, type_, data.as_mut_ptr() as *mut std::ffi::c_void);
        }
        data
    }
}