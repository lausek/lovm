use super::*;

use self::Value::*;

// to support operations on primitive types, lovm wraps them in special `Value` variants.
// this includes `String` which is also used for loading/storing variables, attributes of
// objects, and dispatching function calls.

// table for whole number values
macro_rules! iop_table {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        match (&$lhs, $rhs.cast(&$lhs)) {
            (I(lhs), I(rhs)) => Value::I($op(lhs.clone(), rhs.clone())),
            (I64(lhs), I64(rhs)) => Value::I64($op(lhs.clone(), rhs.clone())),
            (Ref(lhs), Ref(rhs)) => Value::Ref($op(lhs.clone(), rhs).clone()),
            _ => unimplemented!(),
        }
    };
}

// table for numeric values
macro_rules! nop_table {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        match (&$lhs, $rhs.cast(&$lhs)) {
            (F64(lhs), F64(rhs)) => Value::F64($op(lhs.clone(), rhs.clone())),
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
            6 => Value::C('0'),
            7 => Value::Str(String::new()),
            _ => panic!("type index not defined"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::I(n) => format!("{}", n),
            Value::I64(n) => format!("{}", n),
            Value::F64(n) => format!("{}", n),
            Value::Ref(n) => format!("{}", n),
            Value::T(t) => format!("{}", t),
            Value::C(c) => format!("{}", c),
            Value::Str(s) => s.clone(),
        }
    }

    pub fn cast(&self, value: &Value) -> Value {
        self.try_cast(value).unwrap()
    }

    pub fn try_cast(&self, value: &Value) -> Result<Value, ()> {
        match (self, value) {
            (I(_), I(_)) => Ok(self.clone()),
            (I64(n), I(_)) => Ok(Value::I(*n as i8)),
            (F64(n), I(_)) => Ok(Value::I(*n as i8)),
            (Ref(n), I(_)) => Ok(Value::I(*n as i8)),
            (C(c), I(_)) => Ok(Value::I(*c as i8)),
            (T(t), I(_)) => Ok(Value::I(if *t { 1 } else { 0 })),

            (I(n), I64(_)) => Ok(Value::I64(*n as i64)),
            (I64(_), I64(_)) => Ok(self.clone()),
            (F64(n), I64(_)) => Ok(Value::I64(*n as i64)),
            (Ref(n), I64(_)) => Ok(Value::I64(*n as i64)),
            (C(c), I64(_)) => Ok(Value::I64(*c as i64)),
            (T(t), I64(_)) => Ok(Value::I64(if *t { 1 } else { 0 })),

            (I(n), F64(_)) => Ok(Value::F64(*n as f64)),
            (I64(n), F64(_)) => Ok(Value::F64(*n as f64)),
            (F64(_), F64(_)) => Ok(self.clone()),
            (Ref(n), F64(_)) => Ok(Value::F64(*n as f64)),
            (C(c), F64(_)) => Ok(Value::F64((*c as i64) as f64)),
            (T(t), F64(_)) => Ok(Value::F64(if *t { 1. } else { 0. })),

            (I(n), Ref(_)) => Ok(Value::Ref(*n as usize)),
            (I64(n), Ref(_)) => Ok(Value::Ref(*n as usize)),
            (F64(n), Ref(_)) => Ok(Value::Ref(*n as usize)),
            (Ref(_), Ref(_)) => Ok(self.clone()),
            (C(c), Ref(_)) => Ok(Value::Ref(*c as usize)),
            (T(t), Ref(_)) => Ok(Value::Ref(if *t { 1 } else { 0 })),

            (I(n), C(_)) => Ok(Value::C((*n as u8) as char)),
            (I64(n), C(_)) => Ok(Value::C((*n as u8) as char)),
            (F64(n), C(_)) => Ok(Value::C((*n as u8) as char)),
            (Ref(n), C(_)) => Ok(Value::C((*n as u8) as char)),
            (C(_), C(_)) => Ok(self.clone()),
            (T(t), C(_)) => Ok(Value::C(if *t { 't' } else { 'f' })),

            (Str(_), Str(_)) => Ok(self.clone()),
            (Str(_), _) => Err(()), // panic!("no implicit casting from string"),
            (_, Str(_)) => Err(()), // panic!("no implicit casting to string"),

            (T(_), T(_)) => Ok(self.clone()),
            (v, T(_)) => match usize::from(v.clone()) {
                0 => Ok(Value::T(false)),
                1 => Ok(Value::T(true)),
                _ => Err(()), // panic!("invalid numeric value when casting to boolean"),
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

    pub fn add(&self, rhs: &Self) -> Self {
        nop_table!(self, rhs, (|l, r| l + r))
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        nop_table!(self, rhs, (|l, r| l - r))
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        nop_table!(self, rhs, (|l, r| l * r))
    }

    pub fn div(&self, rhs: &Self) -> Self {
        nop_table!(self, rhs, (|l, r| l / r))
    }

    pub fn rem(&self, rhs: &Self) -> Self {
        nop_table!(self, rhs, (|l, r| l % r))
    }

    pub fn neg(&self) -> Self {
        match *self {
            I(v) => Value::I(-v),
            I64(v) => Value::I64(-v),
            F64(v) => Value::F64(-v),
            T(v) => Value::T(!v),
            C(_) => panic!("cannot negate char"),
            Ref(_) | Str(_) => panic!("cannot negate unsigned number"),
        }
    }

    pub fn shl(&self, rhs: &Self) -> Self {
        iop_table!(self, rhs, (|l, r| l << r))
    }

    pub fn shr(&self, rhs: &Self) -> Self {
        iop_table!(self, rhs, (|l, r| l >> r))
    }

    pub fn and(&self, rhs: &Self) -> Self {
        match (&self, rhs.cast(&self)) {
            (T(lhs), T(rhs)) => Value::T(*lhs & rhs),
            _ => iop_table!(self, rhs, (|l, r| l & r)),
        }
    }

    pub fn or(&self, rhs: &Self) -> Self {
        match (&self, rhs.cast(&self)) {
            (T(lhs), T(rhs)) => Value::T(*lhs | rhs),
            _ => iop_table!(self, rhs, (|l, r| l | r)),
        }
    }

    pub fn xor(&self, rhs: &Self) -> Self {
        match (&self, rhs.cast(&self)) {
            (T(lhs), T(rhs)) => Value::T(*lhs ^ rhs),
            _ => iop_table!(self, rhs, (|l, r| l ^ r)),
        }
    }
}

impl std::cmp::PartialEq for Value {
    fn eq(&self, rhs: &Self) -> bool {
        // TODO: comparing values for equality must check the argument; add comparison for losely
        // typed
        //let rhs = rhs.try_cast(&self);
        //if rhs.is_err() {
        //    return false;
        //}
        match (&self, &rhs) {
            (I(lhs), I(rhs)) => lhs == rhs,
            (I64(lhs), I64(rhs)) => lhs == rhs,
            (F64(lhs), F64(rhs)) => lhs == rhs,
            (Ref(lhs), Ref(rhs)) => lhs == rhs,
            (T(lhs), T(rhs)) => lhs == rhs,
            (Str(lhs), Str(rhs)) => lhs == rhs,
            _ => false,
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
            _ => unreachable!(),
        }
    }
}
