use super::*;

use self::Value::*;

// table for whole number values
macro_rules! iop_table {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        match ($lhs, $rhs.cast(&$lhs)) {
            (I(lhs), I(rhs)) => Value::I($op(lhs, rhs)),
            (I64(lhs), I64(rhs)) => Value::I64($op(lhs, rhs)),
            (Ref(lhs), Ref(rhs)) => Value::Ref($op(lhs, rhs)),
            _ => unimplemented!(),
        }
    };
}

// table for numeric values
macro_rules! nop_table {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        match ($lhs, $rhs.cast(&$lhs)) {
            (F64(lhs), F64(rhs)) => Value::F64($op(lhs, rhs)),
            _ => iop_table!($lhs, $rhs, $op),
        }
    };
}

macro_rules! pow {
    ($lhs:expr, $rhs:expr) => {{
        let ex = $rhs.abs() as u32;
        if $rhs.is_negative() {
            1 / $lhs.pow(ex)
        } else {
            $lhs.pow(ex)
        }
    }};
}

macro_rules! powf {
    ($lhs:expr, $rhs:expr) => {{
        if $rhs.is_sign_negative() {
            1. / $lhs.powf(-$rhs)
        } else {
            $lhs.powf($rhs)
        }
    }};
}

impl Value {
    pub fn from_type(idx: usize) -> Value {
        match idx {
            1 => Value::I(0),
            2 => Value::I64(0),
            3 => Value::F64(0.),
            4 => Value::Ref(0),
            5 => Value::T(false),
            _ => panic!("type index not defined"),
        }
    }

    pub fn cast(&self, value: &Value) -> Value {
        match (self, value) {
            (I(_), I(_)) => *self,
            (I64(n), I(_)) => Value::I(*n as i8),
            (F64(n), I(_)) => Value::I(*n as i8),
            (Ref(n), I(_)) => Value::I(*n as i8),
            (T(t), I(_)) => Value::I(if *t { 1 } else { 0 }),

            (I(n), I64(_)) => Value::I64(*n as i64),
            (I64(_), I64(_)) => *self,
            (F64(n), I64(_)) => Value::I64(*n as i64),
            (Ref(n), I64(_)) => Value::I64(*n as i64),
            (T(t), I64(_)) => Value::I64(if *t { 1 } else { 0 }),

            (I(n), F64(_)) => Value::F64(*n as f64),
            (I64(n), F64(_)) => Value::F64(*n as f64),
            (F64(_), F64(_)) => *self,
            (Ref(n), F64(_)) => Value::F64(*n as f64),
            (T(t), F64(_)) => Value::F64(if *t { 1. } else { 0. }),

            (I(n), Ref(_)) => Value::Ref(*n as usize),
            (I64(n), Ref(_)) => Value::Ref(*n as usize),
            (F64(n), Ref(_)) => Value::Ref(*n as usize),
            (Ref(_), Ref(_)) => *self,
            (T(t), Ref(_)) => Value::Ref(if *t { 1 } else { 0 }),

            (v, T(_)) => match usize::from(*v) {
                0 => Value::T(false),
                1 => Value::T(true),
                _ => panic!("invalid numeric value when casting to boolean"),
            },
        }
    }

    pub fn pow(&self, rhs: &Value) -> Value {
        match (self, rhs.cast(&self)) {
            (I(lhs), I(rhs)) => Value::I(pow!(lhs, rhs)),
            (I64(lhs), I64(rhs)) => Value::I64(pow!(lhs, rhs)),
            (F64(lhs), F64(rhs)) => Value::F64(powf!(*lhs, rhs)),
            (Ref(lhs), Ref(rhs)) => Value::Ref(lhs.pow(rhs as u32)),
            _ => unimplemented!(),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self {
        nop_table!(self, rhs, (|l, r| l + r))
    }
}

impl std::ops::Sub for Value {
    type Output = Value;
    fn sub(self, rhs: Self) -> Self {
        nop_table!(self, rhs, (|l, r| l - r))
    }
}

impl std::ops::Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self {
        nop_table!(self, rhs, (|l, r| l * r))
    }
}

impl std::ops::Div for Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Self {
        nop_table!(self, rhs, (|l, r| l / r))
    }
}

impl std::ops::Rem for Value {
    type Output = Value;
    fn rem(self, rhs: Self) -> Self {
        nop_table!(self, rhs, (|l, r| l % r))
    }
}

impl std::ops::Neg for Value {
    type Output = Value;
    fn neg(self) -> Self {
        match self {
            I(v) => Value::I(-v),
            I64(v) => Value::I64(-v),
            F64(v) => Value::F64(-v),
            Ref(_) => panic!("cannot negate unsigned number"),
            T(v) => Value::T(!v),
        }
    }
}

impl std::ops::Shl for Value {
    type Output = Value;
    fn shl(self, rhs: Self) -> Self {
        iop_table!(self, rhs, (|l, r| l << r))
    }
}

impl std::ops::Shr for Value {
    type Output = Value;
    fn shr(self, rhs: Self) -> Self {
        iop_table!(self, rhs, (|l, r| l >> r))
    }
}

impl std::ops::BitAnd for Value {
    type Output = Value;
    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs.cast(&self)) {
            (T(lhs), T(rhs)) => Value::T(lhs & rhs),
            _ => iop_table!(self, rhs, (|l, r| l & r)),
        }
    }
}

impl std::ops::BitOr for Value {
    type Output = Value;
    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs.cast(&self)) {
            (T(lhs), T(rhs)) => Value::T(lhs | rhs),
            _ => iop_table!(self, rhs, (|l, r| l | r)),
        }
    }
}

impl std::ops::BitXor for Value {
    type Output = Value;
    fn bitxor(self, rhs: Self) -> Self {
        match (self, rhs.cast(&self)) {
            (T(lhs), T(rhs)) => Value::T(lhs ^ rhs),
            _ => iop_table!(self, rhs, (|l, r| l ^ r)),
        }
    }
}

impl std::cmp::PartialEq for Value {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs.cast(&self)) {
            (I(lhs), I(rhs)) => *lhs == rhs,
            (I64(lhs), I64(rhs)) => *lhs == rhs,
            (F64(lhs), F64(rhs)) => *lhs == rhs,
            (Ref(lhs), Ref(rhs)) => *lhs == rhs,
            (T(lhs), T(rhs)) => *lhs == rhs,
            _ => unimplemented!(),
        }
    }
}

impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        match (self, rhs.cast(&self)) {
            (I(lhs), I(rhs)) => Some(lhs.cmp(&rhs)),
            (I64(lhs), I64(rhs)) => Some(lhs.cmp(&rhs)),
            (F64(lhs), F64(rhs)) => lhs.partial_cmp(&rhs),
            (Ref(lhs), Ref(rhs)) => Some(lhs.cmp(&rhs)),
            (T(lhs), T(rhs)) => Some(lhs.cmp(&rhs)),
            _ => unimplemented!(),
        }
    }
}
