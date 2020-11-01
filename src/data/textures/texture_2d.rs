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
    , dimension:(usize,usize)
    , format:&TextureFormat
    , data: &[T]
    , data_format:&TextureFormat) -> Self {
        let mut texture = Self::new(context);
        texture.set_data(dimension, &format, data, &data_format);
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
        self.bind();
        unsafe {
            let (color, ty)     = data_format.get_format_type();
            let internal_format = format.internal_format() as i32;
            let width           = dimensions.0 as i32;
            let height          = dimensions.1 as i32;
            gl::TexImage2D(self.type_(),0,internal_format,width,height,0,color,ty,data.as_ptr() as *const std::ffi::c_void);
        }
    }

    /// Gets a copy of the data on the GPU.
    pub fn data<T>(&self) -> Vec<T> {
        Vec::new()
        //FIXME: Read pixels.
        // let (width,height)    = self.dimensions();
        // let capacity          = width * height * self.format().color_format().size();
        // let mut data : Vec<T> = Vec::with_capacity(capacity);
        // let gl = &self.gl;
        // unsafe {
        //     data.set_len(capacity);
        //
        //     //FIXME: Pre-create a transfer framebuffer in Context.
        //     let fb = gl.create_framebuffer().expect("Couldn't create Framebuffer");
        //     gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fb));
        //     gl.framebuffer_texture_2d(glow::FRAMEBUFFER,
        //                               glow::COLOR_ATTACHMENT0,
        //                               glow::TEXTURE_2D,
        //                               Some(self.resource()),
        //                               0);
        //
        //     let (format, ty) = self.format().get_format_type();
        //     let pixels       = glow::PixelPackData::Slice(as_u8_mut_slice(data.as_mut()));
        //     let (width, height) = self.dimensions();
        //     //FIXME: glow read_pixels uses &mut [u8], which can't read GL_FLOAT. To be able to read
        //     // GL_FLOAT, it needs to use a ArrayBufferView from a Float32Array.
        //     gl.read_pixels(0, 0, width as i32, height as i32, format, ty, pixels);
        //
        //
        //     gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        //     gl.delete_framebuffer(fb);
        // }
        // data
    }
}