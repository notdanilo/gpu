use crate::{Context, GLContext, Sampler, Image2D};

type ProgramResource = u32;

/// A structure representing a GPU program.
#[derive(Clone)]
pub struct Program {
    pub(crate) gl : GLContext,
    resource      : ProgramResource
}

impl Program {
    /// Creates a new `Program`.
    pub fn new(context: &Context) -> Self {
        // let gl = context.gl_context();
        // let resource = unsafe {
        //     gl::CreateProgram()
        // };
        // Self { gl, resource}
        unimplemented!()
    }

    /// Gets the `ProgramResource` object.
    pub fn resource(&self) -> ProgramResource { self.resource }

// FIXME: These parts were removed because glow uses a minimum set of GL x GLES x WEBGL.
// These functions can be included in a trait which can be implemented for backends that supports it.
//    fn bind_buffer(&mut self, buffer: &Buffer, index: u32) {
//        unsafe {
//            self::BindBufferBase(gl::SHADER_STORAGE_BUFFER, index, buffer.get_id());
//        }
//    }

    //FIXME: Create Sampler1D, Sampler2D and Sampler2D? What would be the benefits of strong types here?
    /// Binds a `Sampler` at `index`.
    pub fn bind_sampler(&self, sampler:&Sampler, index: usize) {
        // unsafe {
        //     gl::ActiveTexture(gl::TEXTURE0 + index as u32);
        //     gl::BindTexture(sampler.image.type_(), sampler.image.internal());
        //     gl::BindSampler(index as u32, sampler.internal());
        //     gl::UseProgram(self.resource());
        //     gl::Uniform1i(index as i32, index as i32);
        // }
        unimplemented!()
    }

    /// Binds a `bool` to the specified `index`.
    pub fn bind_bool(&self, value: bool, index: usize) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::Uniform1i(index as i32, value as i32);
        // }
        unimplemented!()
    }

    /// Binds a `bvec2` to the specified `index`.
    pub fn bind_bvec2(&self, value: (bool, bool), index: usize) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::Uniform2i(index as i32, value.0 as i32, value.1 as i32);
        // }
        unimplemented!()
    }

    /// Binds a `bvec3` to the specified `index`.
    pub fn bind_bvec3(&self, value: (bool, bool, bool), index: usize) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::Uniform3i(index as i32, value.0 as i32, value.1 as i32, value.2 as i32);
        // }
        unimplemented!()
    }

    /// Binds a `bvec4` to the specified `index`.
    pub fn bind_bvec4(&self, value: (bool, bool, bool, bool), index: usize) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::Uniform4i(index as i32, value.0 as i32, value.1 as i32, value.2 as i32, value.3 as i32);
        // }
        unimplemented!()
    }

    /// Binds a `f32` to the specified `index`.
    pub fn bind_f32(&self, value: f32, index: usize) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::Uniform1f(index as i32, value);
        // }
        unimplemented!()
    }

    /// Binds a `vec2` to the specified `index`.
    pub fn bind_vec2(&self, value:(f32, f32), index: usize) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::Uniform2f(index as i32, value.0, value.1);
        // }
        unimplemented!()
    }

    /// Binds a `vec3` to the specified `index`.
    pub fn bind_vec3(&self, value: (f32, f32, f32), index: usize) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::Uniform3f(index as i32, value.0, value.1, value.2);
        // }
        unimplemented!()
    }

    /// Binds a `vec4` to the specified `index`.
    pub fn bind_vec4(&self, value: (f32, f32, f32, f32), index: usize) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::Uniform4f(index as i32, value.0, value.1, value.2, value.3);
        // }
        unimplemented!()
    }

    // FIXME: This function is not safe. v needs to have 4 * 4 = 16 f32s.
    /// Binds a `mat4` to the specified `index` and determine if it should be `transpose`d.
    pub fn uniform_mat4(&mut self, location: usize, transpose: bool, v: &[f32]) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::UniformMatrix4fv(location as i32, 1, transpose as u8, v.as_ptr());
        // }
        unimplemented!()
    }

    /// Binds an `i32` to the specified `index`.
    pub fn bind_i32(&self, value: i32, index: usize) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::Uniform1i(index as i32, value);
        // }
        unimplemented!()
    }

    /// Binds an `ivec2` to the specified `index`.
    pub fn bind_ivec2(&self, value: (i32, i32), index: usize) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::Uniform2i(index as i32, value.0, value.1);
        // }
        unimplemented!()
    }

    /// Binds an `ivec3` to the specified `index`.
    pub fn bind_ivec3(&self, value: (i32, i32, i32), index: usize) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::Uniform3i(index as i32, value.0, value.1, value.2);
        // }
        unimplemented!()
    }

    /// Binds an `ivec4` to the specified `index`.
    pub fn bind_ivec4(&self, ivec4: (i32, i32, i32, i32), index: usize) {
        // unsafe {
        //     gl::UseProgram(self.resource());
        //     gl::Uniform4i(index as i32, ivec4.0, ivec4.1, ivec4.2, ivec4.3);
        // }
        unimplemented!()
    }

    /// Binds a 2D `image` to the specified `index`.
    pub fn bind_image_2d(&self, image: &Image2D, index: usize) {
       // unsafe {
       //     gl::UseProgram(self.resource());
       //     gl::ActiveTexture(gl::TEXTURE0 + index as u32);
       //     gl::BindTexture(image.type_(), image.internal());
       //     gl::BindImageTexture(index as u32, image.internal(), 0, gl::FALSE, 0, gl::READ_WRITE, image.format().internal_format());
       //     gl::Uniform1i(index as i32, index as i32);
       // }
        unimplemented!()
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        // unsafe {
        //     gl::DeleteProgram(self.resource());
        // }
        unimplemented!()
    }
}