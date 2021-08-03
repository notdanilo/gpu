mod sampling_wrapping;
mod sampling_interpolation;

pub use sampling_wrapping::*;
pub use sampling_interpolation::*;

use crate::{Context, GLContext, Image};

type SamplerResource = u32;

/// A `Sampler` is responsible for sampler values from a texture. It supports filtering and coordinates wrapping.
pub struct Sampler {
    _gl: GLContext,
    resource : SamplerResource,
    sampling_wrapping: SamplingWrapping,
    sampling_interpolation: SamplingInterpolation,
    pub(crate) image: Image,
}

impl Sampler {
    /// Creates a new `Sampler` with default wrapping as `REPEAT` and filtering as `NEAREST`.
    pub fn new(context:&Context, image: &Image, sampling_wrapping: SamplingWrapping, sampling_interpolation: SamplingInterpolation) -> Self {
        // let (minification, magnification) = sampling_interpolation.get_internal_minification_magnification();
        // let (x_wrapping, y_wrapping, z_wrapping) = sampling_wrapping.get_internal();
        // let image = image.clone();
        //
        // let gl = context.gl_context();
        // let resource = unsafe {
        //     let mut resource = 0;
        //     gl::CreateSamplers(1, &mut resource);
        //     gl::SamplerParameteri(resource, gl::TEXTURE_WRAP_S, x_wrapping);
        //     gl::SamplerParameteri(resource, gl::TEXTURE_WRAP_T, y_wrapping);
        //     gl::SamplerParameteri(resource, gl::TEXTURE_WRAP_R, z_wrapping);
        //     gl::SamplerParameteri(resource, gl::TEXTURE_MIN_FILTER, minification);
        //     gl::SamplerParameteri(resource, gl::TEXTURE_MAG_FILTER, magnification);
        //     resource
        // };
        // Self { _gl: gl, resource, sampling_interpolation, sampling_wrapping, image }
        unimplemented!()
    }

    /// Gets `SamplingWrapping`.
    pub fn sampling_wrapping(&self) -> SamplingWrapping {
        self.sampling_wrapping
    }

    /// Gets `SamplingInterpolation`.
    pub fn sampling_interpolation(&self) -> SamplingInterpolation {
        self.sampling_interpolation
    }

    /// Gets `SamplerResource`.
    pub(crate) fn internal(&self) -> SamplerResource {
        self.resource
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        // unsafe {
        //     gl::DeleteSamplers(1, &self.internal());
        // }
        unimplemented!()
    }
}