pub mod code;
pub mod macros;
pub mod op;
pub mod unit;

pub use code::*;
pub use macros::*;
pub use op::*;
pub use unit::*;

use super::*;

pub type BuildResult<T> = Result<T, ()>;

#[derive(PartialEq)]
enum Access {
    Read,
    Write,
}

pub enum BranchTarget {
    Index(usize),
    Block(CodeBuilder),
}

impl From<usize> for BranchTarget {
    fn from(from: usize) -> Self {
        BranchTarget::Index(from)
    }
}

impl<T> From<T> for BranchTarget
where
    T: Into<CodeBuilder>,
{
    fn from(from: T) -> Self {
        BranchTarget::Block(from.into())
    }
}
