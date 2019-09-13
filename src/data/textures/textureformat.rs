use crate::ComponentFormat;
use crate::ColorFormat;

// ref https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexImage2D.xhtml
#[derive(Clone)]
pub struct TextureFormat(pub ColorFormat, pub ComponentFormat);

impl TextureFormat {
    pub fn new(color: ColorFormat, component: ComponentFormat) -> Self {
        TextureFormat(color, component)
    }

    pub fn get_color_format(&self) -> &ColorFormat { &self.0 }
    pub fn get_component_format(&self) -> &ComponentFormat { &self.1 }
}

impl TextureFormat {
    pub fn get_internal_format(&self) -> u32 {
        let (color, component) = (&self.0, &self.1);
        match color {
            ColorFormat::RGBA => match component {
                ComponentFormat::U8 => gl::RGBA8,
                ComponentFormat::U16 => gl::RGBA16UI,
                ComponentFormat::U32 => gl::RGBA32UI,
                ComponentFormat::I8 => gl::RGBA8I,
                ComponentFormat::I16 => gl::RGBA16I,
                ComponentFormat::I32 => gl::RGBA32I,
                ComponentFormat::F16 => gl::RGBA16F,
                ComponentFormat::F32 => gl::RGBA32F
            },
            ColorFormat::RGB => match component {
                ComponentFormat::U8 => gl::RGB8,
                ComponentFormat::U16 => gl::RGB16UI,
                ComponentFormat::U32 => gl::RGB32UI,
                ComponentFormat::I8 => gl::RGB8I,
                ComponentFormat::I16 => gl::RGB16I,
                ComponentFormat::I32 => gl::RGB32I,
                ComponentFormat::F16 => gl::RGB16F,
                ComponentFormat::F32 => gl::RGB32F
            },
            ColorFormat::RG => match component {
                ComponentFormat::U8 => gl::RG8,
                ComponentFormat::U16 => gl::RG16UI,
                ComponentFormat::U32 => gl::RG32UI,
                ComponentFormat::I8 => gl::RG8I,
                ComponentFormat::I16 => gl::RG16I,
                ComponentFormat::I32 => gl::RG32I,
                ComponentFormat::F16 => gl::RG16F,
                ComponentFormat::F32 => gl::RG32F
            },
            ColorFormat::R => match component {
                ComponentFormat::U8 => gl::R8,
                ComponentFormat::U16 => gl::R16UI,
                ComponentFormat::U32 => gl::R32UI,
                ComponentFormat::I8 => gl::R8I,
                ComponentFormat::I16 => gl::R16I,
                ComponentFormat::I32 => gl::R32I,
                ComponentFormat::F16 => gl::R16F,
                ComponentFormat::F32 => gl::R32F
            }
        }
    }
    pub fn get_format_type(&self) -> (u32, u32) {
        let format = self.0.get_format();
        let ty = self.1.get_format();
        (format, ty)
    }
}
