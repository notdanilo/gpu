use crate::Context;

use crate::data::as_u8_slice;
use crate::data::as_u8_mut_slice;

use glow::HasContext;

use crate::TextureFormat;
use crate::ColorFormat;
use crate::ComponentFormat;
use crate::Texture;

use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Texture2D<'context> {
    #[shrinkwrap(main_field)]
    pub texture : Texture<'context>,
    dimensions : (usize,usize)
}

impl<'context> Texture2D<'context> {
    fn new(context:&'context Context) -> Self {
        let format     = TextureFormat::new(ColorFormat::RGBA, ComponentFormat::F32);
        let texture    = Texture::new(context,format,glow::TEXTURE_2D);
        let dimensions = (0,0);
        Self {texture,dimensions}
    }

    pub fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    pub fn allocate
    (context:&'context Context, dimension:(usize,usize), format:&TextureFormat) -> Self {
        let mut texture = Self::new(context);
        texture.reallocate(dimension, &format);
        texture
    }

    pub fn from_data<T>
    ( context:&'context Context
    , dimension:(usize,usize)
    , format:&TextureFormat
    , data: &[T]
    , data_format:&TextureFormat) -> Self {
        let mut texture = Self::new(context);
        texture.set_data(dimension, &format, data, &data_format);
        texture
    }

    pub fn reallocate(&mut self, dimensions: (usize, usize), format: &TextureFormat) {
        self.dimensions = dimensions;
        let gl          = &self.context.gl;
        self.format     = format.clone();
        self.bind();
        unsafe {
            let tex_type        = self.typ();
            let internal_format = format.get_internal_format();
            gl.tex_storage_2d(tex_type, 1, internal_format, dimensions.0 as i32, dimensions.1 as i32);
        }
    }

    pub fn set_data<T>(&mut self, dimensions: (usize, usize), format: &TextureFormat, data: &[T], data_format: &TextureFormat) {
        self.dimensions = dimensions;
        let gl          = &self.context.gl;
        self.format     = format.clone();
        self.bind();
        unsafe {
            let (color, ty)     = data_format.get_format_type();
            let internal_format = format.get_internal_format() as i32;
            let width           = dimensions.0 as i32;
            let height          = dimensions.1 as i32;
            let pixels          = Some(as_u8_slice(data));
            gl.tex_image_2d(self.typ(),0,internal_format,width,height,0,color,ty,pixels);
        }
    }

    pub fn get_data<T>(&self) -> Vec<T> {
        let gl                = &self.context.gl;
        let (width,height)    = self.dimensions();
        let capacity          = width * height * self.format().get_color_format().get_size();
        let mut data : Vec<T> = Vec::with_capacity(capacity);
        unsafe {
            data.set_len(capacity);

            /// FIXME: Pre-create a transfer framebuffer in Context.
            let fb = gl.create_framebuffer().expect("Couldn't create Framebuffer");
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fb));
            gl.framebuffer_texture_2d(glow::FRAMEBUFFER,
                                      glow::COLOR_ATTACHMENT0,
                                      glow::TEXTURE_2D,
                                      Some(self.resource()),
                                      0);

            let (format, ty) = self.format().get_format_type();
            let pixels       = as_u8_mut_slice(data.as_mut());
            let (width, height) = self.dimensions();
            gl.read_pixels(0, 0, width as i32, height as i32, format, ty, pixels);

            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            gl.delete_framebuffer(fb);
        }
        data
    }
}