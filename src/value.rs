use self::Value::*;

#[derive(Clone, Copy, Debug)]
pub enum Value {
    I(i8),
    U(u8),
    I64(i64),
    U64(u64),
    T(bool),
    // TODO: Usize(u8) needed?
    // TODO: add str?
}

impl Value {
    pub fn coalesce(&self, value: &Value) -> Value {
        match (self, value) {
            (U(_), I(_)) | (I(_), I(_)) => *self,
            (U64(_), I64(_)) | (I64(_), I64(_)) => *self,
            (I(s), U(_)) => Value::U(*s as u8),
            (U(_), U(_)) => *self,
            (I64(s), U64(_)) => Value::U64(*s as u64),
            (U64(_), U64(_)) => *self,
            _ => panic!("cannot coalesce from `{:?}` to `{:?}`", self, value),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs.coalesce(&self)) {
            (I(lhs), I(rhs)) => Value::I(lhs + rhs),
            (U(lhs), U(rhs)) => Value::U(lhs + rhs),
            (I64(lhs), I64(rhs)) => Value::I64(lhs + rhs),
            (U64(lhs), U64(rhs)) => Value::U64(lhs + rhs),
            _ => unimplemented!(),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Value;
    fn sub(self, rhs: Self) -> Self {
        match (self, rhs.coalesce(&self)) {
            (I(lhs), I(rhs)) => Value::I(lhs - rhs),
            (U(lhs), U(rhs)) => Value::U(lhs - rhs),
            (I64(lhs), I64(rhs)) => Value::I64(lhs - rhs),
            (U64(lhs), U64(rhs)) => Value::U64(lhs - rhs),
            _ => unimplemented!(),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self {
        match (self, rhs.coalesce(&self)) {
            (I(lhs), I(rhs)) => Value::I(lhs * rhs),
            (U(lhs), U(rhs)) => Value::U(lhs * rhs),
            (I64(lhs), I64(rhs)) => Value::I64(lhs * rhs),
            (U64(lhs), U64(rhs)) => Value::U64(lhs * rhs),
            _ => unimplemented!(),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Self {
        match (self, rhs.coalesce(&self)) {
            (I(lhs), I(rhs)) => Value::I(lhs / rhs),
            (U(lhs), U(rhs)) => Value::U(lhs / rhs),
            (I64(lhs), I64(rhs)) => Value::I64(lhs / rhs),
            (U64(lhs), U64(rhs)) => Value::U64(lhs / rhs),
            _ => unimplemented!(),
        }
    }
}

impl std::ops::BitAnd for Value {
    type Output = Value;
    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs.coalesce(&self)) {
            (I(lhs), I(rhs)) => Value::I(lhs & rhs),
            (U(lhs), U(rhs)) => Value::U(lhs & rhs),
            (I64(lhs), I64(rhs)) => Value::I64(lhs & rhs),
            (U64(lhs), U64(rhs)) => Value::U64(lhs & rhs),
            (T(lhs), T(rhs)) => Value::T(lhs & rhs),
            _ => unimplemented!(),
        }
    }
}

impl std::ops::BitOr for Value {
    type Output = Value;
    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs.coalesce(&self)) {
            (I(lhs), I(rhs)) => Value::I(lhs | rhs),
            (U(lhs), U(rhs)) => Value::U(lhs | rhs),
            (I64(lhs), I64(rhs)) => Value::I64(lhs | rhs),
            (U64(lhs), U64(rhs)) => Value::U64(lhs | rhs),
            (T(lhs), T(rhs)) => Value::T(lhs | rhs),
            _ => unimplemented!(),
        }
    }
}

impl std::ops::BitXor for Value {
    type Output = Value;
    fn bitxor(self, rhs: Self) -> Self {
        match (self, rhs.coalesce(&self)) {
            (I(lhs), I(rhs)) => Value::I(lhs ^ rhs),
            (U(lhs), U(rhs)) => Value::U(lhs ^ rhs),
            (I64(lhs), I64(rhs)) => Value::I64(lhs ^ rhs),
            (U64(lhs), U64(rhs)) => Value::U64(lhs ^ rhs),
            (T(lhs), T(rhs)) => Value::T(lhs ^ rhs),
            _ => unimplemented!(),
        }
    }
}

impl std::cmp::PartialEq for Value {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs.coalesce(&self)) {
            (I(lhs), I(rhs)) => *lhs == rhs,
            (U(lhs), U(rhs)) => *lhs == rhs,
            (I64(lhs), I64(rhs)) => *lhs == rhs,
            (U64(lhs), U64(rhs)) => *lhs == rhs,
            (T(lhs), T(rhs)) => *lhs == rhs,
            _ => unimplemented!(),
        }
    }
}

impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        match (self, rhs.coalesce(&self)) {
            (I(lhs), I(rhs)) => Some(lhs.cmp(&rhs)),
            (U(lhs), U(rhs)) => Some(lhs.cmp(&rhs)),
            (I64(lhs), I64(rhs)) => Some(lhs.cmp(&rhs)),
            (U64(lhs), U64(rhs)) => Some(lhs.cmp(&rhs)),
            (T(lhs), T(rhs)) => Some(lhs.cmp(&rhs)),
            _ => unimplemented!(),
        }
    }
}

impl std::convert::From<Value> for usize {
    fn from(v: Value) -> usize {
        match v {
            I(n) => n as usize,
            U(n) => n as usize,
            I64(n) => n as usize,
            U64(n) => n as usize,
            _ => unimplemented!(),
        }
    }
}
