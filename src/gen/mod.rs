pub mod func;
pub mod module;
pub mod op;

pub use func::*;
pub use module::*;
pub use op::*;

use super::*;

pub type BuildResult<T> = Result<T, ()>;

#[derive(PartialEq)]
enum Access {
    Read,
    Write,
    Append,
}

pub enum BranchTarget {
    Index(usize),
    Block(FunctionBuilder),
}

impl From<usize> for BranchTarget {
    fn from(from: usize) -> Self {
        BranchTarget::Index(from)
    }
}

impl<T> From<T> for BranchTarget
where
    T: Into<FunctionBuilder>,
{
    fn from(from: T) -> Self {
        BranchTarget::Block(from.into())
    }
}
