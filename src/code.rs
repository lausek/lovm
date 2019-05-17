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
pub type CodeBlock = Vec<Instruction>;

pub type Program = Module;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Module {
    pub space: Space,
    pub inner: Vec<(Name, CodeObject)>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            space: Space::new(),
            inner: vec![],
        }
    }

    pub fn get(&self, name: &Name) -> Option<&CodeObject> {
        for (sname, co) in self.inner.iter() {
            if sname == name {
                return Some(co);
            }
        }
        None
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (name, co) in self.inner.iter() {
            writeln!(f, "\t{}:\t{}", name, co)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CodeObject {
    pub argc: usize,
    pub space: Space,
    pub inner: CodeBlock,
}

impl CodeObject {
    pub fn new() -> Self {
        Self {
            argc: 0,
            space: Space::new(),
            inner: CodeBlock::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Space {
    pub consts: Vec<Value>,
    pub locals: Vec<Name>,
    pub globals: Vec<Name>,
}

impl Space {
    pub fn new() -> Self {
        Self {
            consts: vec![],
            locals: vec![],
            globals: vec![],
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

    CmpEq,
    CmpNe, // actually short for `CmpEq; Not`
    CmpGe,
    CmpGt,
    CmpLe,
    CmpLt,

    Jmp(usize),
    Jt(usize),
    Jf(usize),

    Cpush(usize), // push constant
    Lpush(usize), // push local value
    Lpop(usize),  // pop to local
    Lcall(usize),
    Gpush(usize), // push global value
    Gpop(usize),  // pop to global
    Gcall(usize),

    Cast(usize),
    Int(usize),
    Ret,
    Pusha,
    Popa,
}

impl Instruction {
    pub fn arg(&self) -> Option<usize> {
        match self {
            Instruction::Int(c)
            | Instruction::Cast(c)
            | Instruction::Jmp(c)
            | Instruction::Jt(c)
            | Instruction::Jf(c)
            | Instruction::Cpush(c)
            | Instruction::Lpush(c)
            | Instruction::Lpop(c)
            | Instruction::Lcall(c)
            | Instruction::Gpush(c)
            | Instruction::Gpop(c)
            | Instruction::Gcall(c) => Some(*c),
            _ => None,
        }
    }

    pub fn arg_mut(&mut self) -> Option<&mut usize> {
        match self {
            Instruction::Int(c)
            | Instruction::Cast(c)
            | Instruction::Jmp(c)
            | Instruction::Jt(c)
            | Instruction::Jf(c)
            | Instruction::Cpush(c)
            | Instruction::Lpush(c)
            | Instruction::Lpop(c)
            | Instruction::Lcall(c)
            | Instruction::Gpush(c)
            | Instruction::Gpop(c)
            | Instruction::Gcall(c) => Some(c),
            _ => None,
        }
    }

    pub fn arguments(&self) -> usize {
        match self {
            Instruction::Int(_)
            | Instruction::Cast(_)
            | Instruction::Jmp(_)
            | Instruction::Jt(_)
            | Instruction::Jf(_)
            | Instruction::Cpush(_)
            | Instruction::Lpush(_)
            | Instruction::Lpop(_)
            | Instruction::Lcall(_)
            | Instruction::Gpush(_)
            | Instruction::Gpop(_)
            | Instruction::Gcall(_) => 1,

            Instruction::Inc
            | Instruction::Dec
            | Instruction::CmpEq
            | Instruction::CmpNe
            | Instruction::CmpGe
            | Instruction::CmpGt
            | Instruction::CmpLe
            | Instruction::CmpLt
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
            | Instruction::Popa => 0,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
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

    pub fn code(&self) -> &CodeObject {
        self.inner
            .iter()
            .find(|(name, _)| name == "main")
            .map(|(_, code)| code)
            .unwrap()
    }

    pub fn slots(&self) -> &Vec<(Name, CodeObject)> {
        &self.inner
    }

    pub fn slots_mut(&mut self) -> &mut Vec<(Name, CodeObject)> {
        &mut self.inner
    }
}
