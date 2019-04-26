use super::*;

pub type Sequence = Vec<Operation>;

macro_rules! derive_constructor {
    ($ty:path, $name:ident) => {
        impl Operation {
            pub fn $name() -> Self {
                Operation::new($ty)
            }
        }
    };
}

// TODO: operations must be redeclared here (code in asm project has already solved such a problem)
#[derive(Clone, Debug, PartialEq)]
pub enum OperationType {
    Ass,
    Debug,

    Call,
    Ret,
    Push,
    Pop,

    Cmp,
    Jmp,
    Jeq,
    Jne,
    Jge,
    Jgt,
    Jle,
    Jlt,

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

impl Operation {
    pub fn call(fname: &str) -> Self {
        Operation::new(OperationType::Call).op(fname).end()
    }
}

derive_constructor!(OperationType::Ass, ass);
derive_constructor!(OperationType::Debug, debug);
derive_constructor!(OperationType::Ret, ret);
derive_constructor!(OperationType::Push, push);
derive_constructor!(OperationType::Pop, pop);

derive_constructor!(OperationType::Cmp, cmp);
derive_constructor!(OperationType::Jmp, jmp);
derive_constructor!(OperationType::Jeq, jeq);
derive_constructor!(OperationType::Jne, jne);
derive_constructor!(OperationType::Jge, jge);
derive_constructor!(OperationType::Jgt, jgt);
derive_constructor!(OperationType::Jle, jle);
derive_constructor!(OperationType::Jlt, jlt);

derive_constructor!(OperationType::Add, add);
derive_constructor!(OperationType::Sub, sub);
derive_constructor!(OperationType::Mul, mul);
derive_constructor!(OperationType::Div, div);
derive_constructor!(OperationType::Rem, rem);
derive_constructor!(OperationType::Pow, pow);
derive_constructor!(OperationType::Neg, neg);
derive_constructor!(OperationType::And, and);
derive_constructor!(OperationType::Or, or);
derive_constructor!(OperationType::Xor, xor);
derive_constructor!(OperationType::Shl, shl);
derive_constructor!(OperationType::Shr, shr);

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
            //OperationType::Ret => Some(Instruction::Ret),
            //OperationType::Cmp => Some(Instruction::Cmp),
            //OperationType::Jmp => Some(Instruction::Jmp),
            //OperationType::Jeq => Some(Instruction::Jeq),
            //OperationType::Jne => Some(Instruction::Jne),
            //OperationType::Jge => Some(Instruction::Jge),
            //OperationType::Jgt => Some(Instruction::Jgt),
            //OperationType::Jle => Some(Instruction::Jle),
            //OperationType::Jlt => Some(Instruction::Jlt),
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

    pub fn var<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<Name>,
    {
        self.ops.push(Operand::Name(name.into().to_string()));
        self
    }

    pub fn op<T>(&mut self, op: T) -> &mut Self
    where
        T: Into<Operand>,
    {
        self.ops.push(op.into());
        self
    }

    pub fn end(&self) -> Self {
        self.clone()
    }

    pub fn target(&self) -> Option<&Operand> {
        self.ops.get(0)
    }

    pub fn ops(&self) -> impl Iterator<Item = &Operand> {
        self.ops.iter()
    }

    pub fn rest(&self) -> impl Iterator<Item = &Operand> {
        // skip first item as it is the target
        self.ops().skip(1)
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
            Operand::Name(n) | Operand::Const(Value::Str(n)) => n,
            _ => unimplemented!(),
        }
    }

    pub fn as_const(&self) -> &Value {
        match self {
            Operand::Const(v) => v,
            _ => unimplemented!(),
        }
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
