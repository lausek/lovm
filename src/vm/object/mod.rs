use super::*;

pub mod array;
pub mod dict;
pub mod pool;

pub use self::array::*;
pub use self::dict::*;
pub use self::pool::*;

pub type ObjectRef = Box<dyn ObjectProtocol>;

pub trait ObjectProtocol
where
    Self: std::fmt::Debug,
{
    fn call(&mut self, _: &mut Vm);
    fn as_indexable(&mut self) -> Result<&mut dyn Indexable, ()> {
        Err(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub assoc: Option<UnitRef>,
    pub inner: ObjectKind,
}

impl Object {
    pub fn new_value_assoc(assoc: UnitRef) -> Self {
        Self {
            assoc: Some(assoc),
            inner: ObjectKind::Value(Value::I(0)),
        }
    }

    /*
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
    */

    pub fn as_indexable(&mut self) -> Result<&mut dyn Indexable, ()> {
        match &mut self.inner {
            ObjectKind::Array(array) => Ok(array as &mut dyn Indexable),
            ObjectKind::Dict(dict) => Ok(dict as &mut dyn Indexable),
            _ => Err(()),
        }
    }

    pub fn lookup(&self, key: &Value) -> Option<CodeObjectRef> {
        match (&self.assoc, key) {
            (Some(module), Value::Str(name)) => {
                let module: &Unit = module.borrow();
                module.get(name)
            }
            (_, _) => None,
        }
    }
}

impl ObjectProtocol for Object {
    fn call(&mut self, _: &mut Vm) {}
    fn as_indexable(&mut self) -> Result<&mut dyn Indexable, ()> {
        Err(())
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
