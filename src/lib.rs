//! # GPU
//! An ergonomic GPU API.
//!
//! ## Examples
//! Better examples will be provided in the future. For now, please check the [tests](https://github.com/notdanilo/gpu/tree/master/tests).

//TODO: Define a new Error type and get rid of expect/unwrap calls.
//FIXME: We need to type the same documentation for every Context implementation. Maybe we should
//create a documented trait for it and implement the trait for each backend.
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

mod prelude;

mod context;
mod data;
mod code;

pub use data::*;
pub use code::*;
pub use context::*;

