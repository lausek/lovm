// the bytecode definition of lovm

use crate::value::*;

pub type CodeBlock = Vec<Code>;
pub type Instruction = usize;

#[derive(Clone, Copy, Debug)]
pub enum Code {
    Instruction(Instruction),
    Value(Value),
}
