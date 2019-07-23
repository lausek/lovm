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
pub type Offsets = Vec<(usize, BranchTarget)>;

#[derive(PartialEq)]
enum Access {
    Read,
    Write,
}

#[derive(Debug, PartialEq)]
pub enum BranchTarget {
    Index(usize),
    Location(BranchLocation),
    Block(CodeBuilder),
}

#[derive(Debug, PartialEq)]
pub enum BranchLocation {
    Start,
    End,
    Relative(usize),
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

impl From<BranchLocation> for BranchTarget {
    fn from(from: BranchLocation) -> Self {
        BranchTarget::Location(from)
    }
}
