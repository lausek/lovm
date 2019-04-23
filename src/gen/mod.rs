pub mod func;
pub mod module;
pub mod op;
pub mod seq;

pub use func::*;
pub use module::*;
pub use op::*;
pub use seq::*;

use super::*;

use std::collections::HashMap;

type Set<T> = HashMap<T, ()>;
pub type BuildResult<T> = Result<T, ()>;

// TODO: export functionality for generating lovm programs
