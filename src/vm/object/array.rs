use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Array(Vec<Value>);

impl Array {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn code(&self) -> &Vec<Value> {
        &self.0
    }

    pub fn code_mut(&mut self) -> &mut Vec<Value> {
        &mut self.0
    }
}

impl From<Vec<Value>> for Array {
    fn from(from: Vec<Value>) -> Self {
        Self(from)
    }
}

impl ObjectProtocol for Array {
    fn lookup(&self, key: &Value) -> Option<ObjectMethod> {
        match key.to_string().as_ref() {
            "len" => Some(ObjectMethod::Native),
            _ => None,
        }
    }

    fn call(&mut self, name: &Name) -> Result<Option<Value>, ()> {
        match name.as_ref() {
            "len" => Ok(Some(Value::from(self.0.len()))),
            _ => Err(()),
        }
    }

    fn as_indexable(&mut self) -> Result<&mut dyn Indexable, ()> {
        Ok(self as &mut dyn Indexable)
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
