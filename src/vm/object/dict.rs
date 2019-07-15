use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Dict(HashMap<Value, Value>);

impl Dict {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn inner(&self) -> &HashMap<Value, Value> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut HashMap<Value, Value> {
        &mut self.0
    }
}

impl ObjectProtocol for Dict {
    fn invoke(&mut self, _: &mut Vm) {}
}

impl Indexable for Dict {
    fn getk(&self, key: &Value) -> Option<&Value> {
        self.0.get(key)
    }

    fn setk(&mut self, key: &Value, val: Value) {
        self.0.insert(key.clone(), val);
    }

    fn append(&mut self, val: Value) {
        let len = self.0.len();
        self.0.insert(Value::I64(len as i64), val);
    }
}
