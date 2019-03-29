// the bytecode definition of lovm
//

use crate::value::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type CodeBlock = Vec<Code>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub(crate) codeblock: CodeBlock,
    // FIXME: `HashMap` probably is the reason why compiled
    // code files are relatively big in size
    pub(crate) labels: Vec<(String, usize)>,
}

impl Program {
    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(&self)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }

    pub fn with_code(codeblock: CodeBlock) -> Self {
        Self {
            codeblock,
            labels: vec![],
        }
    }

    pub fn code(&self) -> &CodeBlock {
        &self.codeblock
    }

    pub fn labels(&self) -> &Vec<(String, usize)> {
        &self.labels
    }

    pub fn labels_mut(&mut self) -> &mut Vec<(String, usize)> {
        &mut self.labels
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Code {
    Instruction(Instruction),
    // TODO: this should be a variant of `Value`
    Ref(usize),
    Register(Register),
    Value(Value),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[repr(u8)]
pub enum Instruction {
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
    And,
    Or,
    Xor,

    Cmp,
    Jmp,
    Jeq,
    Jne,
    Jge,
    Jgt,
    Jle,
    Jlt,

    Mov,
    Call,
    Ret,
    Push,
    Pop,
    Pusha,
    Popa,
}

impl Instruction {
    pub fn arguments(&self) -> usize {
        match self {
            Instruction::Add
            | Instruction::Sub
            | Instruction::Mul
            | Instruction::Div
            | Instruction::Rem
            | Instruction::Pow
            | Instruction::And
            | Instruction::Or
            | Instruction::Xor
            | Instruction::Cmp
            | Instruction::Mov => 2,

            Instruction::Inc
            | Instruction::Dec
            | Instruction::Jmp
            | Instruction::Jeq
            | Instruction::Jne
            | Instruction::Jge
            | Instruction::Jgt
            | Instruction::Jle
            | Instruction::Jlt
            | Instruction::Call
            | Instruction::Push
            | Instruction::Pop => 1,

            Instruction::Ret | Instruction::Pusha | Instruction::Popa | _ => 0,
        }
    }
}

impl std::str::FromStr for Instruction {
    type Err = &'static str;
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match from {
            "inc" => Ok(Instruction::Inc),
            "dec" => Ok(Instruction::Dec),
            "add" => Ok(Instruction::Add),
            "sub" => Ok(Instruction::Sub),
            "mul" => Ok(Instruction::Mul),
            "div" => Ok(Instruction::Div),
            "rem" => Ok(Instruction::Rem),
            "pow" => Ok(Instruction::Pow),
            "and" => Ok(Instruction::And),
            "or" => Ok(Instruction::Or),
            "xor" => Ok(Instruction::Xor),
            "cmp" => Ok(Instruction::Cmp),
            "jmp" => Ok(Instruction::Jmp),
            "jeq" => Ok(Instruction::Jeq),
            "jne" => Ok(Instruction::Jne),
            "jge" => Ok(Instruction::Jge),
            "jgt" => Ok(Instruction::Jgt),
            "jle" => Ok(Instruction::Jle),
            "jlt" => Ok(Instruction::Jlt),
            "mov" => Ok(Instruction::Mov),
            "call" => Ok(Instruction::Call),
            "ret" => Ok(Instruction::Ret),
            "push" => Ok(Instruction::Push),
            "pop" => Ok(Instruction::Pop),
            "pusha" => Ok(Instruction::Pusha),
            "popa" => Ok(Instruction::Popa),
            _ => Err("not supported"),
        }
    }
}

macro_rules! program {
    {$($inx:expr $(,$reg:ident)* $(,#$c:expr)?;)*} => {{
        use crate::code::Instruction::*;
        use crate::code::Register::*;
        use crate::value::Value::*;
        let codeblock = vec![$(
            crate::code::Code::Instruction($inx)
            $(,
                crate::code::Code::Register($reg)
             )*
            $(,
                crate::code::Code::Value($c)
             )?
        ),*];
        crate::code::Program::with_code(codeblock)
    }}
}
