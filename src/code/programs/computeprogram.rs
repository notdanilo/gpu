use crate::code::Program;
use crate::code::ComputeShader;
use crate::Resource;
use gl::types::*;
use std::ptr;

pub struct ComputeProgram {
    id : u32
}

impl Program for ComputeProgram {
    fn get_id(&self) -> u32 { self.id }
}

impl ComputeProgram {
    pub fn new(compute_shader : ComputeShader) -> Result<ComputeProgram, String> {
        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, compute_shader.get_id());
            gl::LinkProgram(id);

            // Check for linking errors
            let mut success = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as i32 {
                let mut len = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let mut info_log = Vec::with_capacity(len as usize);
                info_log.set_len((len as usize) - 1); // -1 to skip trialing null character

                gl::GetProgramInfoLog(id, len, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                return Err(String::from_utf8(info_log).unwrap());
            }
        }

        Ok(Self {
            id
        })
    }

    pub fn compute(&self, groups: (usize, usize, usize)) {
        unsafe {
            gl::UseProgram(self.get_id());
            gl::DispatchCompute(groups.0 as u32, groups.1 as u32, groups.2 as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ContextBuilder, ContextDisplay, initialize, ComputeShader, Program, ComputeProgram};
    #[test]
    fn write_to_buffer() {
        use crate::Buffer;

        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let mut context = context_builder.build();

        context.make_current().unwrap();
        initialize(|symbol| context.get_proc_address(symbol) as *const _);

        let shader = ComputeShader::new(r#"
        #version 460 compatibility

        layout(binding = 0) buffer Vertices {
         float Positions[];
        };

        layout(local_size_x = 1, local_size_y = 1, local_size_z = 1 ) in;

        void main() {
            uint globalId = gl_GlobalInvocationID.x;
            Positions[gl_GlobalInvocationID.x] = float(globalId);
        }
        "#).unwrap();

        let mut program = ComputeProgram::new(shader).unwrap();

        let length = 256;
        let size = length * 4;

        let buffer = Buffer::allocate(size);
        assert_eq!(buffer.get_size(), size);
        program.bind_buffer(&buffer, 0);

        program.compute((length, 1, 1));

        let mut expected_data : Vec<f32> = Vec::new();
        for x in 0..length {
            expected_data.push(x as f32);
        }
        let data_out : Vec<f32> = buffer.get_data();
        assert_eq!(data_out, expected_data);
    }

    #[test]
    fn write_to_texture2d() {
        use crate::{Texture2D, TextureFormat, ColorFormat, ComponentFormat};

        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let mut context = context_builder.build();

        context.make_current().unwrap();
        initialize(|symbol| context.get_proc_address(symbol) as *const _);

        let shader = ComputeShader::new(r#"
        #version 460 compatibility

        layout (binding = 0, r32f) uniform image2D image;

        layout(local_size_x = 1, local_size_y = 1, local_size_z = 1 ) in;

        void main() {
            ivec2 p = ivec2(gl_GlobalInvocationID.xy);\
            imageStore(image, p, vec4(float(p.x + p.y * 16), 0.0, 0.0, 0.0));\
        }"#).unwrap();

        let mut program = ComputeProgram::new(shader).unwrap();

        let dimension = (16, 16);
        let format = TextureFormat(ColorFormat::R, ComponentFormat::F32);

        let texture = Texture2D::allocate(dimension, &format);
        assert_eq!(texture.get_dimension(), dimension);
        program.bind_image(&texture, 0);

        program.compute((dimension.0, dimension.1, 1));

        let mut expected_data : Vec<f32> = Vec::new();
        for y in 0..dimension.0 {
            for x in 0..dimension.1 {
                expected_data.push((x + y * dimension.0) as f32);
            }
        }
        let data_out : Vec<f32> = texture.get_data();
        assert_eq!(data_out, expected_data);
    }

    #[test]
    fn write_to_texture3d() {
        use crate::Texture3D;
        use crate::TextureFormat;
        use crate::ColorFormat;
        use crate::ComponentFormat;
        use crate::ComputeShader;
        use crate::Program;
        use crate::ComputeProgram;

        let context_builder = ContextBuilder::new().with_display(ContextDisplay::None);
        let mut context = context_builder.build();

        context.make_current().unwrap();
        initialize(|symbol| context.get_proc_address(symbol) as *const _);

        let shader = ComputeShader::new(r#"
        #version 460 compatibility

        layout (binding = 0, r32f) uniform image3D image;

        layout(local_size_x = 1, local_size_y = 1, local_size_z = 1 ) in;

        void main() {
            ivec3 p = ivec3(gl_GlobalInvocationID.xyz);\
            float value = float(p.x + (p.y + p.z * 2) * 2);\
            imageStore(image, p, vec4(value));\
        }"#).unwrap();

        let mut program = ComputeProgram::new(shader).unwrap();

        let dimension = (2, 2, 2);
        let format = TextureFormat(ColorFormat::RGBA, ComponentFormat::F32);

        let texture = Texture3D::allocate(dimension, &format);
        assert_eq!(texture.get_dimension(), dimension);
        program.bind_image(&texture, 0);

        program.compute(dimension);

        let mut expected_data : Vec<f32> = Vec::new();
        for z in 0..dimension.2 {
            for y in 0..dimension.2 {
                for x in 0..dimension.1 {
                    expected_data.push((x + (y + z * 2) * 2) as f32);
                    expected_data.push((x + (y + z * 2) * 2) as f32);
                    expected_data.push((x + (y + z * 2) * 2) as f32);
                    expected_data.push((x + (y + z * 2) * 2) as f32);
                }
            }
        }
        let data_out : Vec<f32> = texture.get_data();
        assert_eq!(data_out, expected_data);
    }
}
