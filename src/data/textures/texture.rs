use crate::Resource;

use crate::TextureFormat;

pub trait Texture : Resource {
    fn get_type(&self) -> u32;
    fn get_format(&self) -> &TextureFormat;
}
