use super::*;

pub mod array;
pub mod dict;
pub mod pool;

pub use self::array::*;
pub use self::dict::*;
pub use self::pool::*;

pub type ObjectRef = Rc<RefCell<dyn ObjectProtocol>>;

pub enum ObjectMethod {
    Virtual(CodeObjectRef),
    // will be implemented in `call`
    Native,
}

pub trait ObjectProtocol
where
    Self: std::fmt::Debug,
{
    fn lookup(&self, _: &Value) -> Option<ObjectMethod> {
        None
    }

    // TODO: add params
    fn call(&mut self, _: &Name) -> Result<Option<Value>, ()> {
        unimplemented!()
    }

    fn as_indexable(&mut self) -> Result<&mut dyn Indexable, ()> {
        Err(())
    }
}

impl ObjectProtocol for Object {
    fn lookup(&self, key: &Value) -> Option<ObjectMethod> {
        self.assoc
            .as_ref()
            .unwrap()
            .0
            .get(&key.to_string())
            .and_then(|cb| Some(ObjectMethod::Virtual(cb)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub assoc: Option<UnitRef>,
    pub inner: Vec<Value>,
}

impl Object {
    pub fn new_value_assoc(assoc: UnitRef) -> Self {
        Self {
            assoc: Some(assoc),
            inner: vec![],
        }
    }
}

// special trait to improve performance on array/dict
pub trait Indexable: std::fmt::Debug {
    // short for "get key"
    fn getk(&self, _: &Value) -> Option<&Value>;
    // short for "set key"
    fn setk(&mut self, _: &Value, _: Value);
    fn append(&mut self, _: Value);
}
