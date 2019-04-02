use self::Value::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Value {
    I(i8),
    U(u8),
    I64(i64),
    U64(u64),
    Ref(usize),
    T(bool),
    // TODO: add str?
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

impl std::str::FromStr for Value {
    type Err = String;
    fn from_str(from: &str) -> Result<Value, Self::Err> {
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
