use crate::Type;
use crate::ColorFormat;

// ref https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexImage2D.xhtml
/// Texture format representation with color format and component format.
#[derive(Clone)]
pub struct TextureFormat(pub ColorFormat, pub Type);

impl TextureFormat {
    /// Creates a new `TextureFormat`.
    pub fn new(color: ColorFormat, component: Type) -> Self {
        TextureFormat(color, component)
    }

    /// Gets the `ColorFormat`.
    pub fn color_format(&self) -> &ColorFormat { &self.0 }

    /// Gets the `ComponentFormat`.
    pub fn component_format(&self) -> &Type { &self.1 }
}

impl TextureFormat {
    /// Gets the internal OpenGL format.
    pub fn internal_format(&self) -> u32 {
        let (color, component) = (&self.0, &self.1);
        match color {
            ColorFormat::RGBA => match component {
                Type::U8  => glow::RGBA8,
                Type::U16 => glow::RGBA16UI,
                Type::U32 => glow::RGBA32UI,
                Type::I8  => glow::RGBA8I,
                Type::I16 => glow::RGBA16I,
                Type::I32 => glow::RGBA32I,
                Type::F16 => glow::RGBA16F,
                Type::F32 => glow::RGBA32F
            },
            ColorFormat::RGB => match component {
                Type::U8  => glow::RGB8,
                Type::U16 => glow::RGB16UI,
                Type::U32 => glow::RGB32UI,
                Type::I8  => glow::RGB8I,
                Type::I16 => glow::RGB16I,
                Type::I32 => glow::RGB32I,
                Type::F16 => glow::RGB16F,
                Type::F32 => glow::RGB32F
            },
            ColorFormat::RG => match component {
                Type::U8  => glow::RG8,
                Type::U16 => glow::RG16UI,
                Type::U32 => glow::RG32UI,
                Type::I8  => glow::RG8I,
                Type::I16 => glow::RG16I,
                Type::I32 => glow::RG32I,
                Type::F16 => glow::RG16F,
                Type::F32 => glow::RG32F
            },
            ColorFormat::R => match component {
                Type::U8  => glow::R8,
                Type::U16 => glow::R16UI,
                Type::U32 => glow::R32UI,
                Type::I8  => glow::R8I,
                Type::I16 => glow::R16I,
                Type::I32 => glow::R32I,
                Type::F16 => glow::R16F,
                Type::F32 => glow::R32F
            }
        }
    }

    /// Gets format and type.
    pub fn get_format_type(&self) -> (u32, u32) {
        let format = self.0.get_format();
        let ty = self.1.format();
        (format, ty)
    }
}
