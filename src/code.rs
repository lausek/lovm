// the bytecode definition of lovm
//
//

use crate::value::*;

pub type CodeBlock = Vec<Code>;

#[derive(Clone, Copy, Debug)]
pub enum Code {
    Instruction(Instruction),
    Value(Value),
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Instruction {
    Null = 0,
    Store,
    Push,
}
