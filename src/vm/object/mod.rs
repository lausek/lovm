use super::*;

pub mod array;
pub mod dict;
pub mod pool;

pub use self::array::*;
pub use self::dict::*;
pub use self::pool::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub assoc: Option<Module>,
    pub inner: ObjectKind,
}

impl Object {
    pub fn new_value() -> Self {
        Self {
            assoc: None,
            inner: ObjectKind::Value(Value::I(0)),
        }
    }

    pub fn new_array() -> Self {
        Self {
            assoc: None,
            inner: ObjectKind::Array(Array::new()),
        }
    }

    pub fn new_dict() -> Self {
        Self {
            assoc: None,
            inner: ObjectKind::Dict(Dict::new()),
        }
    }

    pub fn as_indexable(&mut self) -> Result<&mut dyn Indexable, ()> {
        match &mut self.inner {
            ObjectKind::Array(array) => Ok(array as &mut dyn Indexable),
            ObjectKind::Dict(dict) => Ok(dict as &mut dyn Indexable),
            _ => Err(()),
        }
    }

    pub fn lookup(&self, key: &Value) -> Option<&CodeObject> {
        match (&self.assoc, key) {
            (Some(module), Value::Str(name)) => module.get(name),
            (_, _) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectKind {
    Array(Array),
    Dict(Dict),
    Value(Value),
}

// special trait to improve performance on array/dict
pub trait Indexable: std::fmt::Debug {
    // short for "get key"
    fn getk(&self, _: &Value) -> Option<&Value>;
    // short for "set key"
    fn setk(&mut self, _: &Value, _: Value);
    fn append(&mut self, _: Value);
}
