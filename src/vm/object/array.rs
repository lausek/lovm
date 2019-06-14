use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Array(Vec<Value>);

impl Array {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn inner(&self) -> &Vec<Value> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut Vec<Value> {
        &mut self.0
    }
}

impl From<Vec<Value>> for Array {
    fn from(from: Vec<Value>) -> Self {
        Self(from)
    }
}

impl Indexable for Array {
    fn getk(&self, key: &Value) -> Option<&Value> {
        let idx = usize::from(key.cast(&Value::I64(0)));
        self.0.get(idx)
    }

    fn setk(&mut self, key: &Value, val: Value) {
        let idx = usize::from(key.cast(&Value::I64(0)));
        self.0[idx] = val;
    }

    fn append(&mut self, v: Value) {
        self.0.push(v);
    }
}
