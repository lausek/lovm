pub mod func;
pub mod module;
pub mod op;

pub use func::*;
pub use module::*;
pub use op::*;

use super::*;

pub type BuildResult<T> = Result<T, ()>;

pub fn mkref(from: usize) -> Code {
    Code::Value(Value::Ref(from))
}

// TODO: export functionality for generating lovm programs
