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
            .insert(self.last_handle, ObjectKind::Object(Object::new()));
        self.last_handle
    }

    pub fn new_array_handle(&mut self) -> ObjectId {
        self.last_handle += 1;
        self.handles
            .insert(self.last_handle, ObjectKind::Array(Array::new()));
        self.last_handle
    }

    pub fn dispose_handle(&mut self, id: &ObjectId) {
        self.handles.remove(id);
    }

    pub fn get(&self, id: &ObjectId) -> Option<&ObjectKind> {
        self.handles.get(id)
    }

    pub fn get_handle(&self, id: &ObjectId) -> Option<&dyn ObjectProtocol> {
        self.handles.get(id).and_then(|kind| match kind {
            ObjectKind::Array(array) => Some(array as &ObjectProtocol),
            ObjectKind::Object(object) => Some(object as &ObjectProtocol),
        })
    }

    pub fn get_handle_mut(&mut self, id: &ObjectId) -> Option<&mut dyn ObjectProtocol> {
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

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectKind {
    Array(Array),
    Object(Object),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Array(Vec<Value>);

impl Array {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl ObjectProtocol for Array {
    fn get(&self, key: &Value) -> Option<&Value> {
        let idx = usize::from(key.cast(&Value::I64(0)));
        self.0.get(idx)
    }

    fn set(&mut self, key: &Value, val: Value) {
        let idx = usize::from(key.cast(&Value::I64(0)));
        self.0[idx] = val;
    }

    fn append(&mut self, v: Value) {
        self.0.push(v);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Object(HashMap<Value, Value>);

impl Object {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl ObjectProtocol for Object {
    fn get(&self, key: &Value) -> Option<&Value> {
        self.0.get(key)
    }

    fn set(&mut self, key: &Value, val: Value) {
        self.0.insert(key.clone(), val);
    }

    fn append(&mut self, val: Value) {
        let len = self.0.len();
        self.0.insert(Value::I64(len as i64), val);
    }
}
