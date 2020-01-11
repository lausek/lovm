pub mod block;
pub mod code;
pub mod il;
pub mod macros;
pub mod op;
pub mod unit;

pub use block::*;
pub use code::*;
pub use il::*;
pub use macros::*;
pub use op::*;
pub use unit::*;

use super::*;

pub type BuildResult<T> = Result<T, ()>;
pub type Offsets = Vec<(usize, LinkTarget)>;

#[derive(PartialEq)]
enum Access {
    Read,
    Write,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LinkTarget {
    Index(usize),
    Location(BranchLocation),
    Const(Operand),
    //BlockDef(BlockDef),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BranchLocation {
    Start,
    End,
    Relative(usize),
}

impl From<usize> for LinkTarget {
    fn from(from: usize) -> Self {
        LinkTarget::Index(from)
    }
}

//impl<T> From<T> for LinkTarget
//where
//    T: Into<BlockDef>,
//{
//    fn from(from: T) -> Self {
//        LinkTarget::BlockDef(from.into())
//    }
//}

impl From<BranchLocation> for LinkTarget {
    fn from(from: BranchLocation) -> Self {
        LinkTarget::Location(from)
    }
}

pub fn index_of<T>(ls: &mut Vec<T>, item: &T) -> usize
where
    T: Clone + Eq + std::fmt::Debug,
{
    match ls.iter().position(|a| a == item) {
        Some(idx) => idx,
        _ => {
            ls.push(item.clone());
            ls.len() - 1
        }
    }
}
