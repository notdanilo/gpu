mod context;
mod data;
mod code;

pub use data::*;
pub use code::*;
pub use context::*;

pub trait Resource : Drop {
    fn get_id(&self) -> u32;
}
