//FIXME: Create Wrap struct and Filter struct.

use crate::Context;

use glow::HasContext;

type SamplerResource = <glow::Context as HasContext>::Sampler;

pub struct Sampler<'context> {
    context  : &'context Context,
    resource : SamplerResource
}

impl<'context> Sampler<'context> {
    pub fn new(context:&'context Context) -> Self {
        let gl = &context.gl;
        let resource = unsafe {
            let resource = gl.create_sampler().expect("Couldn't create sampler");
            gl.sampler_parameter_i32(resource, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.sampler_parameter_i32(resource, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            gl.sampler_parameter_i32(resource, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            gl.sampler_parameter_i32(resource, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            resource
        };
        Self {context,resource}
    }

    pub fn resource(&self) -> SamplerResource {
        self.resource
    }
}

impl Drop for Sampler<'_> {
    fn drop(&mut self) {
        unsafe {
            self.context.gl.delete_sampler(self.resource());
        }
    }
}