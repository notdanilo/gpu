//FIXME: Create Wrap struct and Filter struct.

use crate::prelude::*;
use crate::{Context, WeakContext};

type SamplerResource = <glow::Context as HasContext>::Sampler;

/// A `Sampler` is responsible for sampling values from a texture. It supports filtering and coordinates wrapping.
pub struct Sampler {
    context  : WeakContext,
    resource : SamplerResource
}

impl Sampler {
    /// Creates a new `Sampler` with default wrapping as `REPEAT` and filtering as `NEAREST`.
    pub fn new(context:&Context) -> Self {
        let gl = context.internal_context();
        let resource = unsafe {
            let resource = gl.create_sampler().expect("Couldn't create sampler");
            gl.sampler_parameter_i32(resource, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.sampler_parameter_i32(resource, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            gl.sampler_parameter_i32(resource, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            gl.sampler_parameter_i32(resource, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            resource
        };
        let context = context.weak_ref();
        Self {context,resource}
    }

    /// Gets `SamplerResource`.
    pub fn resource(&self) -> SamplerResource {
        self.resource
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        self.context.upgrade().map(|context| {
            unsafe {
                context.internal_context().delete_sampler(self.resource());
            }
        });
    }
}