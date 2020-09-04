// ref https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glTexImage2D.xhtml

/// Kinds of `ColorFormat`s.
#[derive(Clone)]
pub enum ColorFormat {
    /// Red only `ColorFormat`.
    R,
    /// Red and green `ColorFormat`.
    RG,
    /// Red, green and blue `ColorFormat`.
    RGB,
    /// Red, green, blue and alpha `ColorFormat`.
    RGBA
}

impl ColorFormat {
    /// Creates a `ColorFormat` with `n_components`, where `n_components` can be anything from 0 to
    /// 3. Any value above it will be clipped to 3.
    pub fn components(n_components: usize) -> ColorFormat {
        match n_components {
            0..=1 => ColorFormat::R,
            2     => ColorFormat::RG,
            3     => ColorFormat::RGB,
            _     => ColorFormat::RGBA
        }
    }

    /// Gets the size of the `ColorFormat` in bytes.
    pub fn size(&self) -> usize {
        match self {
            ColorFormat::R    => 1,
            ColorFormat::RG   => 2,
            ColorFormat::RGB  => 3,
            ColorFormat::RGBA => 4
        }
    }

    /// Gets `OpenGL` internal enumeration.
    pub fn get_format(&self) -> u32 {
        match self {
            ColorFormat::R    => glow::RED,
            ColorFormat::RG   => glow::RG,
            ColorFormat::RGB  => glow::RGB,
            ColorFormat::RGBA => glow::RGBA
        }
    }
}
