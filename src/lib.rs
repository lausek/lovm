#![feature(macro_at_most_once_rep)]

// TODO: export functionality for generating lovm programs

#[macro_use]
pub mod code;
pub mod test;
pub mod value;
pub mod vm;

pub use code::*;
pub use value::*;
