use super::*;

pub type Sequence = Vec<Operation>;

// TODO: operations must be redeclared here (code in asm project has already solved such a problem)
#[derive(Clone, Debug, PartialEq)]
pub enum OperationType {
    Ass,
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
}

#[derive(Clone, Debug)]
pub struct Operation {
    ops: Vec<Operand>,
    pub ty: OperationType,
    update: bool,
}

impl Operation {
    pub fn new(ty: OperationType) -> Self {
        Self {
            ops: vec![],
            ty,
            update: false,
        }
    }

    pub fn as_inx(&self) -> Option<Instruction> {
        match self.ty {
            OperationType::Add => Some(Instruction::Add),
            OperationType::Sub => Some(Instruction::Sub),
            OperationType::Mul => Some(Instruction::Mul),
            OperationType::Div => Some(Instruction::Div),
            OperationType::Rem => Some(Instruction::Rem),
            OperationType::Pow => Some(Instruction::Pow),
            OperationType::Neg => Some(Instruction::Neg),
            OperationType::And => Some(Instruction::And),
            OperationType::Or => Some(Instruction::Or),
            OperationType::Xor => Some(Instruction::Xor),
            OperationType::Shl => Some(Instruction::Shl),
            OperationType::Shr => Some(Instruction::Shr),
            _ => None,
        }
    }

    pub fn update(mut self) -> Self {
        self.update = true;
        self
    }

    pub fn is_update(&self) -> bool {
        self.update
    }

    pub fn consts(&self) -> impl Iterator<Item = &Value> {
        self.ops.iter().filter_map(|op| match op {
            Operand::Const(v) => Some(v),
            _ => None,
        })
    }

    pub fn idents(&self) -> impl Iterator<Item = &Name> {
        self.ops.iter().filter_map(|op| match op {
            Operand::Name(name) => Some(name),
            _ => None,
        })
    }

    pub fn op<T>(mut self, op: T) -> Self
    where
        T: Into<Operand>,
    {
        self.ops.push(op.into());
        self
    }

    pub fn target(&self) -> Option<&Operand> {
        self.ops.get(0)
    }

    pub fn ops(&self) -> impl Iterator<Item = &Operand> {
        // skip first item as it is the target
        self.ops.iter().skip(1)
    }
}

#[derive(Clone, Debug)]
pub enum Operand {
    Const(Value),
    Name(Name),
}

impl Operand {
    pub fn as_name(&self) -> &Name {
        match self {
            Operand::Name(n) => n,
            _ => unimplemented!(),
        }
    }
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        Operand::Name(s.into())
    }
}

impl<T> From<T> for Operand
where
    T: Into<Value>,
{
    fn from(v: T) -> Self {
        Operand::Const(v.into())
    }
}
