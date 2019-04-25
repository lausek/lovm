pub mod func;
pub mod module;
pub mod op;

pub use func::*;
pub use module::*;
pub use op::*;

use super::*;

pub type BuildResult<T> = Result<T, ()>;

// TODO: export functionality for generating lovm programs
