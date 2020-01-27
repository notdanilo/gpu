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
                ComponentFormat::U8  => glow::RGBA8,
                ComponentFormat::U16 => glow::RGBA16UI,
                ComponentFormat::U32 => glow::RGBA32UI,
                ComponentFormat::I8  => glow::RGBA8I,
                ComponentFormat::I16 => glow::RGBA16I,
                ComponentFormat::I32 => glow::RGBA32I,
                ComponentFormat::F16 => glow::RGBA16F,
                ComponentFormat::F32 => glow::RGBA32F
            },
            ColorFormat::RGB => match component {
                ComponentFormat::U8  => glow::RGB8,
                ComponentFormat::U16 => glow::RGB16UI,
                ComponentFormat::U32 => glow::RGB32UI,
                ComponentFormat::I8  => glow::RGB8I,
                ComponentFormat::I16 => glow::RGB16I,
                ComponentFormat::I32 => glow::RGB32I,
                ComponentFormat::F16 => glow::RGB16F,
                ComponentFormat::F32 => glow::RGB32F
            },
            ColorFormat::RG => match component {
                ComponentFormat::U8  => glow::RG8,
                ComponentFormat::U16 => glow::RG16UI,
                ComponentFormat::U32 => glow::RG32UI,
                ComponentFormat::I8  => glow::RG8I,
                ComponentFormat::I16 => glow::RG16I,
                ComponentFormat::I32 => glow::RG32I,
                ComponentFormat::F16 => glow::RG16F,
                ComponentFormat::F32 => glow::RG32F
            },
            ColorFormat::R => match component {
                ComponentFormat::U8  => glow::R8,
                ComponentFormat::U16 => glow::R16UI,
                ComponentFormat::U32 => glow::R32UI,
                ComponentFormat::I8  => glow::R8I,
                ComponentFormat::I16 => glow::R16I,
                ComponentFormat::I32 => glow::R32I,
                ComponentFormat::F16 => glow::R16F,
                ComponentFormat::F32 => glow::R32F
            }
        }
    }
    pub fn get_format_type(&self) -> (u32, u32) {
        let format = self.0.get_format();
        let ty = self.1.get_format();
        (format, ty)
    }
}
