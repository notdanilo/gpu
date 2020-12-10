//! Module for sampling interpolation.

/// Available interpolations.
#[derive(Clone,Copy)]
pub enum Interpolation {
    /// Nearest interpolation. Creates a pixelated look.
    Nearest,
    /// Linear interpolation.
    Linear
}

impl Interpolation {
    pub(crate) fn get_internal(&self) -> i32 {
        match self {
            Self::Nearest => gl::NEAREST as i32,
            Self::Linear  => gl::LINEAR as i32
        }
    }
}

/// Sampling interpolation.
#[derive(Clone,Copy)]
pub struct SamplingInterpolation {
    minification: Interpolation,
    magnification: Interpolation
}

impl SamplingInterpolation {
    /// Constructor.
    pub fn new(minification: Interpolation, magnification: Interpolation) -> Self {
        Self { minification, magnification }
    }

    /// Constructs both `minification` and `magnification` with the same `Interpolation`.
    pub fn all(all: Interpolation) -> Self {
        Self {
            minification: all,
            magnification: all
        }
    }

    /// Gets the minification `Interpolation`.
    pub fn minification(&self) -> Interpolation {
        self.minification
    }

    /// Gets the magnification `Interpolation`.
    pub fn magnification(&self) -> Interpolation {
        self.magnification
    }

    pub(crate) fn get_internal_minification_magnification(&self) -> (i32, i32) {
        (self.minification.get_internal(), self.magnification.get_internal())
    }
}