use super::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct VmMemory {
    mem: HashMap<usize, Code>,
}

impl VmMemory {
    pub fn new() -> Self {
        Self {
            mem: HashMap::new(),
        }
    }

    pub fn map(&mut self, bl: &CodeBlock, at: usize) {
        for (off, inx) in bl.iter().enumerate() {
            self.mem.insert(at + off, *inx);
        }
    }
}

impl std::ops::Index<usize> for VmMemory {
    type Output = Code;
    fn index(&self, idx: usize) -> &Code {
        self.mem.get(&idx).unwrap()
    }
}

impl std::ops::IndexMut<usize> for VmMemory {
    fn index_mut(&mut self, idx: usize) -> &mut Code {
        // TODO: check if code is value here or panic (access error)
        if !self.mem.contains_key(&idx) {
            self.mem.insert(idx, Code::Value(Value::I(0)));
        }
        self.mem.get_mut(&idx).unwrap()
    }
}
