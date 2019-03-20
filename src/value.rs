use self::Value::*;

type Numeric = f64;

#[derive(Clone, Copy, Debug)]
pub enum Value {
    I(i8),
    U(u8),
    I64(i64),
    U64(u64),
    T(bool),
    // TODO: add str?
}

impl Value {
    pub fn coalesce(&self, value: &Value) -> Value {
        match (self, value) {
            (I(s), I(n)) => *self,
            (U(s), U(n)) => *self,
            (I64(s), I64(n)) => *self,
            (U64(s), U64(n)) => *self,
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
