//! <a href="https://crates.io/crates/gpu"><img src="https://img.shields.io/crates/v/gpu.svg?label=gpu" alt="crates.io"></a>
//! <a href="https://docs.rs/gpu"><img src="https://docs.rs/gpu/badge.svg" alt="docs.rs"></a>
//! ![Build (MacOS, Linux, Windows)](https://github.com/notdanilo/gpu/workflows/Build%20(MacOS,%20Linux,%20Windows)/badge.svg?branch=master)
//!
//! # GPU
//! An ergonomic GPU API.
//!
//! ## Examples
//! Better examples will be provided in the future. For now, please check the [tests](https://github.com/notdanilo/gpu/tree/master/tests).

//TODO: Define a new Error type and get rid of expect/unwrap calls.
//FIXME: We need to type the same documentation for every Context implementation. Maybe we should
//create a documented trait for it and implement the trait for each backend.
#![warn(missing_docs)]
#![warn(trivial_numeric_casts)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

mod prelude;

mod context;
mod data;
mod code;
mod window;

pub use data::*;
pub use code::*;
pub use context::*;
pub use window::*;
