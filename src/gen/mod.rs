pub mod func;
pub mod module;
pub mod op;

pub use func::*;
pub use module::*;
pub use op::*;

use super::*;

use std::collections::HashMap;

pub type BuildResult<T> = Result<T, ()>;

// this struct receives a list of jump operations and branches
// to the second argument
pub struct Branch {
    jumps: Vec<(Operation, usize)>,
}

impl Branch {
    pub fn new() -> Self {
        Self { jumps: vec![] }
    }
}

// TODO: export functionality for generating lovm programs
