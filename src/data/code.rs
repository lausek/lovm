use super::*;

use serde::*;

// a `CodeObject` is an executable bytecode unit. it holds the local constant values,
// local identifiers, and extern (or global) identifiers. this allows a clear separation
// of data and logic, aswell as a flattening of the bytecode structure due to the
// usement of standardized indexing types (e.g. Loadc(1) loads consts[1] onto the stack).
//
// the use of registers is therefore dropped in favor of a more dynamic and flexible
// data management.
//
// for grouping `CodeObjects` into units, a `Unit` structure is used. it basically contains
// a list of identifiers next to their correspoding `CodeObject` (possible sig: Vec<(Ident, CodeObject)>).
//
// for the generation of lovm programs a library (WIP: module name) is exported.

pub type Name = String;

pub type Code = Protocol<usize>;
pub type CodeBlock = Vec<Code>;

// a program is nothing else than a `Unit` with a method named `main` that will be
// used as entry point of execution.
pub type Program = Unit;

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

    pub fn into_ref(self) -> CodeObjectRef {
        CodeObjectRef::from(self)
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

// the bytecode definition of lovm
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[repr(u8)]
pub enum Protocol<T> {
    Dup,
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

    Jmp(T),
    Jt(T),
    Jf(T),

    CPush(T), // push constant
    LPush(T), // push local value
    LPop(T),  // pop to local
    LCall(T),
    GPush(T), // push global value
    GPop(T),  // pop to global
    GCall(T),

    Cast(T),
    Int(T),
    Ret,
    Pusha,
    Popa,

    // create a new object pushing its handle onto the stack
    ONew(T),
    // create a new array pushing its handle onto the stack
    ONewArray,
    // create a new dict pushing its handle onto the stack
    ONewDict,
    // dispose the last object on stack
    ODispose,
    // use constant at this index for accessing/calling object attributes
    OGet(T),
    OSet(T),
    OCall(T),
    OAppend,
}

impl Code {
    pub fn arg(&self) -> Option<usize> {
        match self {
            Code::Int(c)
            | Code::Cast(c)
            | Code::Jmp(c)
            | Code::Jt(c)
            | Code::Jf(c)
            | Code::CPush(c)
            | Code::LPush(c)
            | Code::LPop(c)
            | Code::LCall(c)
            | Code::GPush(c)
            | Code::GPop(c)
            | Code::GCall(c)
            | Code::ONew(c)
            | Code::OGet(c)
            | Code::OSet(c)
            | Code::OCall(c) => Some(*c),
            _ => None,
        }
    }

    pub fn arg_mut(&mut self) -> Option<&mut usize> {
        match self {
            Code::Int(c)
            | Code::Cast(c)
            | Code::Jmp(c)
            | Code::Jt(c)
            | Code::Jf(c)
            | Code::CPush(c)
            | Code::LPush(c)
            | Code::LPop(c)
            | Code::LCall(c)
            | Code::GPush(c)
            | Code::GPop(c)
            | Code::GCall(c)
            | Code::ONew(c)
            | Code::OGet(c)
            | Code::OSet(c)
            | Code::OCall(c) => Some(c),
            _ => None,
        }
    }

    pub fn arguments(&self) -> usize {
        match self {
            Code::Int(_)
            | Code::Cast(_)
            | Code::Jmp(_)
            | Code::Jt(_)
            | Code::Jf(_)
            | Code::CPush(_)
            | Code::LPush(_)
            | Code::LPop(_)
            | Code::LCall(_)
            | Code::GPush(_)
            | Code::GPop(_)
            | Code::GCall(_)
            | Code::ONew(_)
            | Code::OGet(_)
            | Code::OSet(_)
            | Code::OCall(_) => 1,
            _ => 0,
        }
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Unit {
    pub space: Space,
    pub inner: Vec<(Name, CodeObjectRef)>,
}

impl Unit {
    pub fn new() -> Self {
        Self {
            space: Space::new(),
            inner: vec![],
        }
    }

    // TODO: rename to `lookup` or something like that
    pub fn get(&self, name: &Name) -> Option<CodeObjectRef> {
        for (sname, co) in self.inner.iter() {
            if sname == name {
                return Some(co.clone());
            }
        }
        None
    }

    pub fn set(&mut self, name: &Name, co: CodeObject) {
        if let Some(mut slot) = self.get(name) {
            *slot.get_mut() = co;
        } else {
            self.inner.push((name.clone(), CodeObjectRef::from(co)));
        }
    }

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
        new.set(&"main".into(), co);
        new
    }

    pub fn code(&self) -> CodeObjectRef {
        self.inner
            .iter()
            .find(|(name, _)| name == "main")
            .map(|(_, code)| code.clone())
            .unwrap()
    }

    pub fn slots(&self) -> &Vec<(Name, CodeObjectRef)> {
        &self.inner
    }

    pub fn slots_mut(&mut self) -> &mut Vec<(Name, CodeObjectRef)> {
        &mut self.inner
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Unit(slots: {})", self.inner.len())?;
        for (name, co) in self.inner.iter() {
            writeln!(f, "\t{}:\t{}", name, co)?;
        }
        Ok(())
    }
}
