use super::*;

use std::cmp;

#[derive(Clone, Debug)]
pub struct VmFrame {
    pub locals: Vec<Value>,
    pub cmp: Option<cmp::Ordering>,
}

impl VmFrame {
    pub fn new(argc: usize) -> Self {
        Self {
            locals: (0..argc).map(|_| Value::I(0)).collect(),
            cmp: None,
        }
    }

    pub fn is_jmp_needed(&self, inx: &Instruction) -> bool {
        let cmp = self.cmp.expect("no comparison");
        match inx {
            Instruction::Jeq(_) if cmp == cmp::Ordering::Equal => true,
            Instruction::Jne(_) if cmp != cmp::Ordering::Equal => true,
            Instruction::Jge(_)
                if (cmp == cmp::Ordering::Greater) | (cmp == cmp::Ordering::Equal) =>
            {
                true
            }
            Instruction::Jgt(_) if cmp == cmp::Ordering::Greater => true,
            Instruction::Jle(_) if (cmp == cmp::Ordering::Less) | (cmp == cmp::Ordering::Equal) => {
                true
            }
            Instruction::Jlt(_) if cmp == cmp::Ordering::Less => true,
            Instruction::Jmp(_) => true,
            // no jump will be executed
            _ => false,
        }
    }
}
