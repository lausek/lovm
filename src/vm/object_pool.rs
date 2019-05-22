use super::*;

#[derive(Clone, Debug)]
pub struct ObjectPool {
    last_handle: ObjectId,
    handles: HashMap<ObjectId, ObjectKind>,
}

impl ObjectPool {
    pub fn new() -> Self {
        Self {
            last_handle: 0,
            handles: HashMap::new(),
        }
    }

    pub fn new_handle(&mut self) -> ObjectId {
        self.last_handle += 1;
        self.handles
            .insert(self.last_handle, ObjectKind::Array(Array::new()));
        self.last_handle
    }

    pub fn dispose_handle(&mut self, id: &ObjectId) {
        self.handles.remove(id);
    }

    pub fn get(&self, id: &ObjectId) -> Option<&dyn ObjectProtocol> {
        self.handles.get(id).and_then(|kind| match kind {
            ObjectKind::Array(array) => Some(array as &ObjectProtocol),
            ObjectKind::Object(object) => Some(object as &ObjectProtocol),
        })
    }

    pub fn get_mut(&mut self, id: &ObjectId) -> Option<&mut dyn ObjectProtocol> {
        self.handles.get_mut(id).and_then(|kind| match kind {
            ObjectKind::Array(array) => Some(array as &mut ObjectProtocol),
            ObjectKind::Object(object) => Some(object as &mut ObjectProtocol),
        })
    }
}

pub trait ObjectProtocol: std::fmt::Debug {
    fn get(&self, _: &Value) -> Option<&Value>;
    fn set(&mut self, _: &Value, _: Value);
    fn append(&mut self, _: Value);
}

#[derive(Clone, Debug)]
pub enum ObjectKind {
    Array(Array),
    Object(Object),
}

#[derive(Clone, Debug)]
pub struct Array(Vec<Value>);

impl Array {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl ObjectProtocol for Array {
    fn get(&self, _: &Value) -> Option<&Value> {
        None
    }

    fn set(&mut self, _: &Value, _: Value) {}

    fn append(&mut self, v: Value) {
        self.0.push(v);
    }
}

#[derive(Clone, Debug)]
pub struct Object {}

impl ObjectProtocol for Object {
    fn get(&self, _: &Value) -> Option<&Value> {
        None
    }

    fn set(&mut self, _: &Value, _: Value) {}

    fn append(&mut self, _: Value) {}
}
