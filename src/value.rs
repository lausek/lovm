use self::Value::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Value {
    I(i8),
    I64(i64),
    F64(f64),
    Ref(usize),
    T(bool),
    // TODO: add str?
}

impl std::convert::From<Value> for usize {
    fn from(v: Value) -> usize {
        match v {
            I(n) => n as usize,
            I64(n) => n as usize,
            F64(n) => n as usize,
            Ref(n) => n,
            T(t) => if t {1} else {0},
        }
    }
}

impl std::convert::From<i8> for Value {
    fn from(n: i8) -> Value {
        Value::I(n)
    }
}

impl std::convert::From<i64> for Value {
    fn from(n: i64) -> Value {
        Value::I64(n)
    }
}

impl std::convert::From<f64> for Value {
    fn from(n: f64) -> Value {
        Value::F64(n)
    }
}

impl std::convert::From<usize> for Value {
    fn from(n: usize) -> Value {
        Value::Ref(n)
    }
}

impl std::convert::From<bool> for Value {
    fn from(t: bool) -> Value {
        Value::T(t)
    }
}

impl std::str::FromStr for Value {
    type Err = String;
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match from {
            "true" => Ok(Value::T(true)),
            "false" => Ok(Value::T(false)),
            _ => {
                const MIN: i64 = i8::min_value() as i64;
                const MAX: i64 = i8::max_value() as i64;
                match i64::from_str(from).unwrap() {
                    val @ MIN..=MAX => Ok(Value::I(val as i8)),
                    val => Ok(Value::I64(val)),
                }
            }
        }
    }
}
