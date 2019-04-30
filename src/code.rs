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

    Cmp,
    Jmp(usize),
    Jeq(usize),
    Jne(usize),
    Jge(usize),
    Jgt(usize),
    Jle(usize),
    Jlt(usize),

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
    pub fn set_arg(&mut self, arg: usize) {
        match self {
            Instruction::Int(c)
            | Instruction::Cast(c)
            | Instruction::Jmp(c)
            | Instruction::Jeq(c)
            | Instruction::Jne(c)
            | Instruction::Jge(c)
            | Instruction::Jgt(c)
            | Instruction::Jle(c)
            | Instruction::Jlt(c)
            | Instruction::Cpush(c)
            | Instruction::Lpush(c)
            | Instruction::Lpop(c)
            | Instruction::Lcall(c)
            | Instruction::Gpush(c)
            | Instruction::Gpop(c)
            | Instruction::Gcall(c) => *c = arg,
            _ => unimplemented!(),
        }
    }

    pub fn arguments(&self) -> usize {
        match self {
            Instruction::Int(_)
            | Instruction::Cast(_)
            | Instruction::Jmp(_)
            | Instruction::Jeq(_)
            | Instruction::Jne(_)
            | Instruction::Jge(_)
            | Instruction::Jgt(_)
            | Instruction::Jle(_)
            | Instruction::Jlt(_)
            | Instruction::Cpush(_)
            | Instruction::Lpush(_)
            | Instruction::Lpop(_)
            | Instruction::Lcall(_)
            | Instruction::Gpush(_)
            | Instruction::Gpop(_)
            | Instruction::Gcall(_) => 1,

            Instruction::Inc
            | Instruction::Dec
            | Instruction::Cmp
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

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut it = self.code().inner.iter();
        let mut offset = 0;

        writeln!(f, "program:")?;
        while let Some(inx) = it.next() {
            write!(f, "{:04}: {:?}", offset, inx)?;
            offset += 1;
            for _ in 0..inx.arguments() {
                write!(f, "\t{:?}", it.next().unwrap())?;
                offset += 1;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
