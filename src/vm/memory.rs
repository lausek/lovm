use super::*;

pub struct VmMemory {
    chunk: Vec<Value>,
}

impl VmMemory {
    pub fn new() -> Self {
        Self { chunk: vec![] }
    }
}

impl std::ops::Index<usize> for VmMemory {
    type Output = Value;
    fn index(&self, idx: usize) -> &Value {
        self.chunk.get(idx).unwrap()
    }
}

impl std::ops::IndexMut<usize> for VmMemory {
    fn index_mut(&mut self, idx: usize) -> &mut Value {
        self.chunk.get_mut(idx).unwrap()
    }
}
