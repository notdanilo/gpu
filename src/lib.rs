
extern crate gl;

mod context;
mod data;
mod code;

pub use data::*;
pub use code::*;
pub use context::*;

use std::ffi::c_void;
use std::str;

pub fn initialize<F>(loadfn: F)
where F: FnMut(&'static str) -> *const c_void
{
    gl::load_with(loadfn);
}

pub trait Resource : Drop {
    fn get_id(&self) -> u32;
}
