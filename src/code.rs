// the bytecode definition of lovm

use crate::value::*;

use serde::{Deserialize, Serialize};

// a `CodeObject` is an executable bytecode unit. it holds the local constant values,
// local identifiers, and extern (or global) identifiers. this allows a clear separation
// of data and logic, aswell as a flattening of the bytecode structure due to the
// usement of standardized indexing types (e.g. Loadc(1) loads consts[1] onto the stack).
//
// the use of registers is therefore dropped in favor of a more dynamic and flexible
// data management.
//
// for grouping `CodeObjects` into units, a `Module` structure is used. it basically contains
// a list of identifiers next to their correspoding `CodeObject` (possible sig: Vec<(Ident, CodeObject)>).
//
// for the generation of lovm programs a library (WIP: module name) is exported.

pub type Name = String;
pub type CodeBlock = Vec<Code>;

pub type CodeObject = Space<CodeBlock>;
pub type Module = Space<Vec<(Name, CodeObject)>>;
pub type Program = Module;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Space<T>
where
    T: std::default::Default,
{
    pub consts: Vec<Value>,
    pub locals: Vec<Name>,
    pub globals: Vec<Name>,
    pub inner: T,
}

impl<T> Space<T>
where
    T: std::default::Default,
{
    pub fn new() -> Self {
        Self {
            consts: vec![],
            locals: vec![],
            globals: vec![],
            inner: T::default(),
        }
    }
}

// TODO: modifications for support of constant values
//          - new instruction `Loadc`, loads a constant value onto the stack
//          - add new constant vector to `Program`

/*
TODO: remove; replaced by lovm_asm_lib
macro_rules! program {
    {$($inx:expr $(,$reg:ident)* $(,#$c:expr)?;)*} => {{
        use crate::code::Instruction::*;
        use crate::code::Register::*;
        use crate::value::Value::*;
        let code = vec![$(
            crate::code::Code::Instruction($inx)
            $(,
                crate::code::Code::Register($reg)
             )*
            $(,
                crate::code::Code::Value($c)
             )?
        ),*];
        crate::code::Program::with_code(code)
    }}
}
*/

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Code {
    Instruction(Instruction),
    Register(Register),
    Value(Value),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
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

    Cast,
    Call,
    Int,
    Ret,
    Push,
    Pop,
    Pusha,
    Popa,
}

impl Instruction {
    pub fn arguments(&self) -> usize {
        match self {
            Instruction::Int
            | Instruction::Cast
            | Instruction::Inc
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

            Instruction::Cmp
            | Instruction::Add
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

impl Module {
    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(&self)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }

    pub fn with_code(code: CodeBlock) -> Self {
        let mut new = Self::new();
        let mut co = CodeObject::new();
        co.inner = code;
        new.inner = vec![("main".into(), co)];
        new
    }

    pub fn code(&self) -> &CodeBlock {
        self.inner
            .iter()
            .find(|(name, _)| name == "main")
            .map(|(_, code)| &code.inner)
            .unwrap()
    }

    pub fn slots(&self) -> &Vec<(Name, CodeObject)> {
        &self.inner
    }

    pub fn slots_mut(&mut self) -> &mut Vec<(Name, CodeObject)> {
        &mut self.inner
    }

    // TODO: `labels_*` functions will be dropped as they were meant for static linking (not supported anymore)
    /*
    pub fn labels(&self) -> &Vec<(String, usize)> {
        unimplemented!()
    }

    pub fn labels_mut(&mut self) -> &mut Vec<(String, usize)> {
        unimplemented!()
    }

    pub fn entry_point(&self) -> Option<usize> {
        self.labels()
            .iter()
            .find(|(name, _)| name == "main")
            .map(|(_, off)| *off)
    }
    */
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut it = self.code().iter();
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
