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
            ComponentFormat::U8 | ComponentFormat::I8 => 1,
            ComponentFormat::U16 | ComponentFormat::I16 | ComponentFormat::F16 => 2,
            ComponentFormat::U32 | ComponentFormat::I32 | ComponentFormat::F32 => 4
        }
    }

    pub fn get_format(&self) -> u32 {
        match self {
            ComponentFormat::U8 => gl::UNSIGNED_BYTE,
            ComponentFormat::U16 => gl::UNSIGNED_SHORT,
            ComponentFormat::U32 => gl::UNSIGNED_INT,
            ComponentFormat::I8 => gl::BYTE,
            ComponentFormat::I16 => gl::SHORT,
            ComponentFormat::I32 => gl::INT,
            ComponentFormat::F16 => gl::HALF_FLOAT,
            ComponentFormat::F32 => gl::FLOAT
        }
    }
}
