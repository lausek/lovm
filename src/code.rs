// the bytecode definition of lovm
//
//

use crate::value::*;

pub type CodeBlock = Vec<Code>;

#[derive(Clone, Copy, Debug)]
pub enum Code {
    Instruction(Instruction),
    Ref(usize),
    Register(Register),
    Value(Value),
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,

    Store,

    Push,
}

macro_rules! code {
    {$($inx:expr $(,$reg:ident)* $(,#$c:expr)?;)*} => {{
        use crate::code::Instruction::*;
        use crate::code::Register::*;
        use crate::value::Value::*;
        vec![$(
            crate::code::Code::Instruction($inx),
            $(
                crate::code::Code::Register($reg)
             ),*
            $(,
                crate::code::Code::Value($c)
             )?
        ),*]
    }}
}
