// the bytecode definition of lovm

use crate::value::*;

use serde::{Deserialize, Serialize};

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

pub type CodeBlock = Vec<Code>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub(crate) codeblock: CodeBlock,
    pub(crate) labels: Vec<(String, usize)>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Code {
    Instruction(Instruction),
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

impl std::str::FromStr for Register {
    type Err = ();
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match from.chars().nth(0) {
            Some('A') => Ok(Register::A),
            Some('B') => Ok(Register::B),
            Some('C') => Ok(Register::C),
            Some('D') => Ok(Register::D),
            _ => Err(()),
        }
    }
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
    Neg,
    And,
    Or,
    Xor,
    Shl,
    Shr,

    Cmp,
    Jmp,
    Jeq,
    Jne,
    Jge,
    Jgt,
    Jle,
    Jlt,

    Load,  // pops a ref off the stack, leaving the locations value inplace
    Store, // pops a ref and value off the stack, writing value to location ref

    Coal,
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
            Instruction::Cmp | Instruction::Coal => 2,

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

            Instruction::Add
            | Instruction::Sub
            | Instruction::Mul
            | Instruction::Div
            | Instruction::Rem
            | Instruction::Pow
            | Instruction::Neg
            | Instruction::And
            | Instruction::Or
            | Instruction::Xor
            | Instruction::Shl
            | Instruction::Shr
            | Instruction::Ret
            | Instruction::Pusha
            | Instruction::Popa
            | Instruction::Load
            | Instruction::Store => 0,
        }
    }
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

    pub fn entry_point(&self) -> Option<usize> {
        self.labels()
            .iter()
            .find(|(name, _)| name == "main")
            .map(|(_, off)| *off)
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut it = self.codeblock.iter();
        let mut offset = 0;

        writeln!(f, "program:")?;
        while let Some(inx) = it.next() {
            write!(f, "{:04}: {:?}", offset, inx)?;
            offset += 1;
            match inx {
                Code::Instruction(inx) => {
                    for _ in 0..inx.arguments() {
                        write!(f, "\t{:?}", it.next().unwrap())?;
                        offset += 1;
                    }
                }
                _ => {}
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
