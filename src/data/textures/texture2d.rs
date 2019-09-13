use std::ffi::c_void;

use crate::data::textures::TextureFormat;
use crate::ColorFormat;
use crate::ComponentFormat;
use crate::Texture;
use crate::Resource;

pub struct Texture2D {
    id : u32,
    format: TextureFormat
}

impl Texture2D {
    fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Self {
            id : id,
            format: TextureFormat::new(ColorFormat::RGBA, ComponentFormat::F32)
        }
    }

    pub fn get_dimension(&self) -> (usize, usize) {
        (self.get_width(), self.get_height())
    }

    pub fn get_width(&self) -> usize {
        unsafe {
            let mut width = 0;
            gl::GetTexLevelParameteriv(gl::TEXTURE_2D, 0, gl::TEXTURE_WIDTH, &mut width);
            width as usize
        }
    }

    pub fn get_height(&self) -> usize {
        unsafe {
            let mut height = 0;
            gl::GetTexLevelParameteriv(gl::TEXTURE_2D, 0, gl::TEXTURE_HEIGHT, &mut height);
            height as usize
        }
    }

    pub fn allocate(dimension: (usize, usize), format: &TextureFormat) -> Self {
        let mut texture = Self::new();
        texture.reallocate(dimension, &format);
        texture
    }

    pub fn from_data<T>(dimension: (usize, usize), format: &TextureFormat, data: &[T], data_format: &TextureFormat) -> Self {
        let mut texture = Self::new();
        texture.set_data(dimension, &format, data, &data_format);
        texture
    }

    pub fn reallocate(&mut self, dimension: (usize, usize), format: &TextureFormat) {
        self.format = format.clone();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexStorage2D(gl::TEXTURE_2D, 1, format.get_internal_format(), dimension.0 as i32, dimension.1 as i32);
        }
    }

    pub fn set_data<T>(&mut self, dimension: (usize, usize), format: &TextureFormat, data: &[T], data_format: &TextureFormat) {
        self.format = format.clone();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            let (color, ty) = data_format.get_format_type();
            gl::TexImage2D(gl::TEXTURE_2D, 0, format.get_internal_format() as i32, dimension.0 as i32, dimension.1 as i32, 0, color, ty, &data[0] as *const T as *const c_void);
        }
    }

    pub fn get_data<T>(&self) -> Vec<T> {
        let capacity = self.get_width() * self.get_height() * self.get_format().get_color_format().get_size();
        let mut data : Vec<T> = Vec::with_capacity(capacity);
        unsafe {
            data.set_len(capacity);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.get_id());
            let (format, ty) = self.format.get_format_type();
            gl::GetTexImage(gl::TEXTURE_2D, 0, format, ty, data.as_mut_ptr() as *mut c_void);

        }
        data
    }
}

impl Resource for Texture2D {
    fn get_id(&self) -> u32 { self.id }
}

impl Texture for Texture2D {
    fn get_type(&self) -> u32 { gl::TEXTURE_2D }
    fn get_format(&self) -> &TextureFormat { &self.format }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}


#[test]
fn allocation() {
    let mut context = crate::Context::new();

    context.make_current().unwrap();
    crate::initialize(|symbol| context.get_proc_address(symbol) as *const _);

    let dimension = (123, 321);
    let texture = Texture2D::allocate(dimension, &TextureFormat(ColorFormat::RGBA, ComponentFormat::U8));
    assert_eq!(texture.get_dimension(), dimension);
}

#[test]
fn from_data() {
    let mut context = crate::Context::new();

    context.make_current().unwrap();
    crate::initialize(|symbol| context.get_proc_address(symbol) as *const _);

    let mut data_in : Vec<f32> = Vec::new();
    let dimension = (8, 8);
    let components = 2;
    for x in 0..dimension.0 {
        for y in 0..dimension.1 {
            for z in 1..(components+1) {
                data_in.push((z * (y * dimension.0 + x)) as f32);
            }
        }
    }

    let data_in_format = TextureFormat(ColorFormat::components(components), ComponentFormat::F32);
    let texture = Texture2D::from_data(dimension, &data_in_format, &data_in, &data_in_format);

    assert_eq!(components, texture.get_format().get_color_format().get_size());
    assert_eq!(dimension, texture.get_dimension());

    let data_out = texture.get_data();

    assert_eq!(data_in, data_out);
}
