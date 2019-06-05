use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Array(Vec<Value>);

impl Array {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl IndexProtocol for Array {
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
