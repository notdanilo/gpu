//! Module for sampling wrapping.

/// All the available `Wrapping` methods.
#[derive(Clone,Copy)]
pub enum Wrapping {
    /// Repeats the sample.
    Repeat,
    /// Repeats the sample, but mirroring it at every repetition.
    MirroredRepeat,
    /// Clamps the sampling to the edge.
    ClampToEdge
}

impl Wrapping {
    pub(crate) fn get_internal(&self) -> i32 {
        match self {
            Self::Repeat => gl::REPEAT as i32,
            Self::MirroredRepeat => gl::MIRRORED_REPEAT as i32,
            Self::ClampToEdge => gl::CLAMP_TO_EDGE as i32
        }
    }
}

/// Sampling `Wrapping` over `x`, `y` and `z`.
#[derive(Clone,Copy)]
pub struct SamplingWrapping {
    x: Wrapping,
    y: Wrapping,
    z: Wrapping
}

impl SamplingWrapping {
    /// Constructor.
    pub fn new(x: Wrapping, y: Wrapping, z: Wrapping) -> Self {
        Self { x, y, z }
    }

    /// Constructs `x`, `y` and `z` with the same `Wrapping` setup.
    pub fn all(all: Wrapping) -> Self {
        Self {
            x: all,
            y: all,
            z: all
        }
    }

    /// Gets the `Wrapping` method over `x`.
    pub fn x(&self) -> Wrapping {
        self.x
    }

    /// Gets the `Wrapping` method over `y`.
    pub fn y(&self) -> Wrapping {
        self.y
    }

    /// Gets the `Wrapping` method over `z`.
    pub fn z(&self) -> Wrapping {
        self.z
    }

    pub(crate) fn get_internal(&self) -> (i32, i32, i32) {
        (self.x.get_internal(), self.y.get_internal(), self.z.get_internal())
    }
}