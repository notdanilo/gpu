//FIXME:Fix warnings
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

pub mod context;
mod data;
mod code;

pub use data::*;
pub use code::*;
pub use context::*;