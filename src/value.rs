use self::Value::*;

use serde::{Deserialize, Serialize};

// TODO: change type of `Value::Str(_)` to a String type
//       that can be passed around easily over a StringPool (smth. like Rc<String> or Cow<String>)
// TODO: implement Objects; stored in ObjectPool (requires well-designed memory layout)

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    I(i8),
    I64(i64),
    F64(f64),
    Ref(usize),
    T(bool),
    C(char),
    Str(String),
}

impl std::convert::From<Value> for usize {
    fn from(v: Value) -> usize {
        match v {
            I(n) => n as usize,
            I64(n) => n as usize,
            F64(n) => n as usize,
            Ref(n) => n,
            T(t) => {
                if t {
                    1
                } else {
                    0
                }
            }
            C(c) => c as usize,
            Str(_) => unimplemented!(),
        }
    }
}

impl std::convert::From<i8> for Value {
    fn from(n: i8) -> Value {
        Value::I(n)
    }
}

impl std::convert::From<i32> for Value {
    fn from(n: i32) -> Value {
        Value::I64(n as i64)
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

impl std::convert::From<&str> for Value {
    fn from(s: &str) -> Value {
        Value::Str(s.into())
    }
}

impl std::convert::From<Value> for bool {
    fn from(s: Value) -> bool {
        match s {
            Value::T(t) => t,
            _ => panic!("cannot convert `{:?}` into bool", s),
        }
    }
}

impl std::cmp::Eq for Value {}

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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Value::I(arg) => write!(f, "{}", arg),
            Value::I64(arg) => write!(f, "{}", arg),
            Value::F64(arg) => write!(f, "{}", arg),
            Value::Ref(arg) => write!(f, "{}", arg),
            Value::T(arg) => write!(f, "{}", arg),
            Value::C(arg) => write!(f, "{}", arg),
            Value::Str(arg) => write!(f, "{}", arg),
        }
    }
}
