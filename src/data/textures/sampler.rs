//FIXME: Create Wrap struct and Filter struct.

use crate::{Context, GLContext};

type SamplerResource = u32;

/// A `Sampler` is responsible for sampling values from a texture. It supports filtering and coordinates wrapping.
pub struct Sampler {
    gl       : GLContext,
    resource : SamplerResource
}

impl Sampler {
    /// Creates a new `Sampler` with default wrapping as `REPEAT` and filtering as `NEAREST`.
    pub fn new(context:&Context) -> Self {
        let gl = context.gl_context();
        let resource = unsafe {
            let mut resource = 0;
            gl::CreateSamplers(1, &mut resource);
            gl::SamplerParameteri(resource, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::SamplerParameteri(resource, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::SamplerParameteri(resource, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::SamplerParameteri(resource, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            resource
        };
        Self { gl, resource }
    }

    /// Gets `SamplerResource`.
    pub fn resource(&self) -> SamplerResource {
        self.resource
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteSamplers(1, &self.resource());
        }
    }
}