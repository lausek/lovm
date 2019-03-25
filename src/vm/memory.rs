use super::*;

use std::collections::HashMap;

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
        self.mem.get_mut(&idx).unwrap()
    }
}
