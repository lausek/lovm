use super::*;

use std::cmp;

#[derive(Clone, Debug)]
pub struct VmFrame {
    pub a: Value,
    pub b: Value,
    pub c: Value,
    pub d: Value,
    pub locals: Vec<Value>,
    pub cmp: Option<cmp::Ordering>,
}

impl VmFrame {
    pub fn new(argc: usize) -> Self {
        Self {
            a: Value::I(0),
            b: Value::I(0),
            c: Value::I(0),
            d: Value::I(0),
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

impl std::ops::Index<Register> for VmFrame {
    type Output = Value;
    fn index(&self, idx: Register) -> &Value {
        match idx {
            Register::A => &self.a,
            Register::B => &self.b,
            Register::C => &self.c,
            Register::D => &self.d,
        }
    }
}

impl std::ops::IndexMut<Register> for VmFrame {
    fn index_mut(&mut self, idx: Register) -> &mut Value {
        match idx {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
        }
    }
}
