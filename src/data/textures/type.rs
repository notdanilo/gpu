// ref https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexImage2D.xhtml

/// Types used by the GPU.
#[derive(Clone)]
pub enum Type {
    /// 8bits unsigned integer.
    U8,
    /// 16bits unsigned integer.
    U16,
    /// 32bits unsigned integer.
    U32,
    /// 8bits signed integer.
    I8,
    /// 16bits signed integer.
    I16,
    /// 32bits signed integer.
    I32,
    /// 16bits float.
    F16,
    /// 32bits float.
    F32,
}

impl Type {
    /// Gets the size in bytes.
    pub fn size(&self) -> usize {
        match self {
            Type::U8  | Type::I8 => 1,
            Type::U16 | Type::I16 | Type::F16 => 2,
            Type::U32 | Type::I32 | Type::F32 => 4
        }
    }

    /// Gets `OpenGL` internal reprensetation.
    pub fn format(&self) -> u32 {
        match self {
            Type::U8  => glow::UNSIGNED_BYTE,
            Type::U16 => glow::UNSIGNED_SHORT,
            Type::U32 => glow::UNSIGNED_INT,
            Type::I8  => glow::BYTE,
            Type::I16 => glow::SHORT,
            Type::I32 => glow::INT,
            Type::F16 => glow::HALF_FLOAT,
            Type::F32 => glow::FLOAT
        }
    }
}
