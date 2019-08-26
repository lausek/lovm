use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Dict(HashMap<Value, Value>);

impl Dict {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn code(&self) -> &HashMap<Value, Value> {
        &self.0
    }

    pub fn code_mut(&mut self) -> &mut HashMap<Value, Value> {
        &mut self.0
    }
}

impl ObjectProtocol for Dict {
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
