use super::*;

pub type Sequence = Vec<Operation>;

macro_rules! derive_constructor {
    ($ty:path, $name:ident) => {
        pub fn $name() -> Operation {
            Operation::new($ty)
        }

        impl Operation {
            pub fn $name() -> Self {
                $name()
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq)]
pub enum OperationType {
    Ass,
    Debug,

    Call,
    Ret,
    Push,
    Pop,

    ONew,
    ONewArray,
    ONewDict,
    ODispose,
    OAppend,
    OGet,
    OSet,

    CmpEq,
    CmpNe, // actually short for `CmpEq; Not`
    CmpGe,
    CmpGt,
    CmpLe,
    CmpLt,
    Jmp,
    Jt,
    Jf,

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

pub fn call(fname: &str) -> Operation {
    Operation::new(OperationType::Call).var(fname).end()
}

impl Operation {
    pub fn call(fname: &str) -> Self {
        call(fname)
    }
}

derive_constructor!(OperationType::Ass, ass);
derive_constructor!(OperationType::Debug, debug);
derive_constructor!(OperationType::Ret, ret);
derive_constructor!(OperationType::Push, push);
derive_constructor!(OperationType::Pop, pop);
derive_constructor!(OperationType::ONew, onew);
derive_constructor!(OperationType::ONewArray, onewarray);
derive_constructor!(OperationType::ONewDict, onewdict);
derive_constructor!(OperationType::ODispose, odispose);
derive_constructor!(OperationType::OAppend, oappend);
derive_constructor!(OperationType::OGet, oget);
derive_constructor!(OperationType::OSet, oset);

derive_constructor!(OperationType::CmpEq, cmp_eq);
derive_constructor!(OperationType::CmpNe, cmp_ne);
derive_constructor!(OperationType::CmpGe, cmp_ge);
derive_constructor!(OperationType::CmpGt, cmp_gt);
derive_constructor!(OperationType::CmpLe, cmp_le);
derive_constructor!(OperationType::CmpLt, cmp_lt);

derive_constructor!(OperationType::Jmp, jmp);
derive_constructor!(OperationType::Jt, jt);
derive_constructor!(OperationType::Jf, jf);

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

#[derive(Clone, Debug, PartialEq)]
pub enum OpValue {
    Operand(Operand),
    Operation(Operation),
}

impl std::fmt::Display for OpValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            OpValue::Operand(op) => write!(f, "{}", op),
            OpValue::Operation(op) => write!(f, "{}", op),
        }
    }
}

impl<T: Into<Operand>> From<T> for OpValue {
    fn from(from: T) -> Self {
        OpValue::Operand(from.into())
    }
}

// constructor for arrays (tuples)
impl<T> From<Vec<T>> for OpValue
where
    T: Into<OpValue>,
{
    fn from(from: Vec<T>) -> Self {
        let mut ops = Operation::onewarray();
        for item in from.into_iter() {
            ops.op(item);
        }
        OpValue::Operation(ops)
    }
}

// TODO: this is ugly
// constructor for objects (sets)
impl<T> From<Vec<(Option<T>, T)>> for OpValue
where
    T: Into<OpValue>,
{
    fn from(from: Vec<(Option<T>, T)>) -> Self {
        let mut ops = Operation::onewdict();
        for (key, val) in from.into_iter() {
            if let Some(key) = key {
                ops.op(Operation::oset().op(key).op(val).end());
            } else {
                ops.op(Operation::oappend().op(val).end());
            }
        }
        OpValue::Operation(ops)
    }
}

impl From<Operation> for OpValue {
    fn from(from: Operation) -> Self {
        OpValue::Operation(from)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Operation {
    pub ops: Vec<OpValue>,
    pub ty: OperationType,
}

impl Operation {
    pub fn new(ty: OperationType) -> Self {
        Self { ops: vec![], ty }
    }

    pub fn as_inx(&self) -> Option<Instruction> {
        match self.ty {
            OperationType::CmpEq => Some(Instruction::CmpEq),
            OperationType::CmpNe => Some(Instruction::CmpNe),
            OperationType::CmpGe => Some(Instruction::CmpGe),
            OperationType::CmpGt => Some(Instruction::CmpGt),
            OperationType::CmpLe => Some(Instruction::CmpLe),
            OperationType::CmpLt => Some(Instruction::CmpLt),

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

            OperationType::ODispose => Some(Instruction::ODispose),
            _ => None,
        }
    }

    pub fn consts(&self) -> impl Iterator<Item = &Value> {
        let mut consts = vec![];
        for op in self.ops.iter() {
            match op {
                OpValue::Operand(Operand::Const(v)) => consts.push(v),
                OpValue::Operation(op) => consts.extend(op.consts().collect::<Vec<_>>()),
                _ => {}
            }
        }
        consts.into_iter()
    }

    pub fn idents(&self) -> impl Iterator<Item = &Name> {
        let mut idents = vec![];
        for op in self.ops.iter() {
            match op {
                OpValue::Operand(Operand::Name(name)) => idents.push(name),
                OpValue::Operation(op) => idents.extend(op.idents().collect::<Vec<_>>()),
                _ => {}
            }
        }
        idents.into_iter()
    }

    pub fn var<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<Name>,
    {
        self.ops
            .push(OpValue::Operand(Operand::Name(name.into().to_string())));
        self
    }

    pub fn op<T>(&mut self, op: T) -> &mut Self
    where
        T: Into<OpValue>,
    {
        self.ops.push(op.into());
        self
    }

    pub fn end(&self) -> Self {
        self.clone()
    }

    pub fn target(&self) -> Option<&Operand> {
        self.ops.get(0).and_then(|op| match op {
            OpValue::Operand(target) => Some(target),
            _ => None,
        })
    }

    pub fn ops(&self) -> impl Iterator<Item = &OpValue> {
        self.ops.iter()
    }

    pub fn rest(&self) -> impl Iterator<Item = &OpValue> {
        // skip first item as it is the target
        self.ops().skip(1)
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({:?}", self.ty)?;
        for (i, op) in self.ops().enumerate() {
            if 0 < i {
                write!(f, ",")?;
            } else {
            }
            write!(f, " {}", op)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Const(Value),
    Name(Name),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Operand::Const(c) => write!(f, "{}", c),
            Operand::Name(n) => write!(f, "{}", n),
        }
    }
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
