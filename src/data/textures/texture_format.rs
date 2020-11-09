use crate::Type;
use crate::ColorFormat;

// ref https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexImage2D.xhtml
/// Texture format representation with color format and component format.
#[derive(Clone,Copy)]
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
        // FIXME: Rewrite as:
        //         match (color, component) {
        //             (ColorFormat::RGBA, Type::U8) => gl::RGBA8,
        //             ...
        match color {
            ColorFormat::RGBA => match component {
                Type::U8  => gl::RGBA8,
                Type::U16 => gl::RGBA16UI,
                Type::U32 => gl::RGBA32UI,
                Type::I8  => gl::RGBA8I,
                Type::I16 => gl::RGBA16I,
                Type::I32 => gl::RGBA32I,
                Type::F16 => gl::RGBA16F,
                Type::F32 => gl::RGBA32F
            },
            ColorFormat::RGB => match component {
                Type::U8  => gl::RGB8,
                Type::U16 => gl::RGB16UI,
                Type::U32 => gl::RGB32UI,
                Type::I8  => gl::RGB8I,
                Type::I16 => gl::RGB16I,
                Type::I32 => gl::RGB32I,
                Type::F16 => gl::RGB16F,
                Type::F32 => gl::RGB32F
            },
            ColorFormat::RG => match component {
                Type::U8  => gl::RG8,
                Type::U16 => gl::RG16UI,
                Type::U32 => gl::RG32UI,
                Type::I8  => gl::RG8I,
                Type::I16 => gl::RG16I,
                Type::I32 => gl::RG32I,
                Type::F16 => gl::RG16F,
                Type::F32 => gl::RG32F
            },
            ColorFormat::R => match component {
                Type::U8  => gl::R8,
                Type::U16 => gl::R16UI,
                Type::U32 => gl::R32UI,
                Type::I8  => gl::R8I,
                Type::I16 => gl::R16I,
                Type::I32 => gl::R32I,
                Type::F16 => gl::R16F,
                Type::F32 => gl::R32F
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
