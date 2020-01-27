// ref https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexImage2D.xhtml

#[derive(Clone)]
pub enum ComponentFormat {
    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
    F16,
    F32,
}

impl ComponentFormat {
    pub fn get_size(&self) -> usize {
        match self {
            ComponentFormat::U8  | ComponentFormat::I8 => 1,
            ComponentFormat::U16 | ComponentFormat::I16 | ComponentFormat::F16 => 2,
            ComponentFormat::U32 | ComponentFormat::I32 | ComponentFormat::F32 => 4
        }
    }

    pub fn get_format(&self) -> u32 {
        match self {
            ComponentFormat::U8  => glow::UNSIGNED_BYTE,
            ComponentFormat::U16 => glow::UNSIGNED_SHORT,
            ComponentFormat::U32 => glow::UNSIGNED_INT,
            ComponentFormat::I8  => glow::BYTE,
            ComponentFormat::I16 => glow::SHORT,
            ComponentFormat::I32 => glow::INT,
            ComponentFormat::F16 => glow::HALF_FLOAT,
            ComponentFormat::F32 => glow::FLOAT
        }
    }
}
