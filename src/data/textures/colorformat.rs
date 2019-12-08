// ref https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexImage2D.xhtml

#[derive(Clone)]
pub enum ColorFormat {
    R,
    RG,
    RGB,
    RGBA
}

impl ColorFormat {
    pub fn components(components: usize) -> ColorFormat {
        match components {
            0..=1 => ColorFormat::R,
            2 => ColorFormat::RG,
            3 => ColorFormat::RGB,
            _ => ColorFormat::RGBA
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            ColorFormat::R => 1,
            ColorFormat::RG => 2,
            ColorFormat::RGB => 3,
            ColorFormat::RGBA => 4
        }
    }

    pub fn get_format(&self) -> u32 {
        match self {
            ColorFormat::R => gl::RED,
            ColorFormat::RG => gl::RG,
            ColorFormat::RGB => gl::RGB,
            ColorFormat::RGBA => gl::RGBA
        }
    }
}
