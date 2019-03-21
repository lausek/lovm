use super::*;

use std::cmp;

#[derive(Clone, Copy, Debug)]
pub struct VmRegister {
    a: Value,
    b: Value,
    c: Value,
    d: Value,
    pub(super) cmp: Option<cmp::Ordering>,
}

impl VmRegister {
    pub fn new() -> Self {
        Self {
            a: Value::U(0),
            b: Value::U(0),
            c: Value::U(0),
            d: Value::U(0),
            cmp: None,
        }
    }

    pub fn is_jmp_needed(&self, inx: Instruction) -> bool {
        let cmp = self.cmp.expect("no comparison");
        match inx {
            Instruction::Jeq if cmp == cmp::Ordering::Equal => true,
            Instruction::Jne if cmp != cmp::Ordering::Equal => true,
            Instruction::Jge if (cmp == cmp::Ordering::Greater) | (cmp == cmp::Ordering::Equal) => {
                true
            }
            Instruction::Jgt if cmp == cmp::Ordering::Greater => true,
            Instruction::Jle if (cmp == cmp::Ordering::Less) | (cmp == cmp::Ordering::Equal) => {
                true
            }
            Instruction::Jlt if cmp == cmp::Ordering::Less => true,
            // no jump will be executed
            _ => false,
        }
    }
}

impl std::ops::Index<Register> for VmRegister {
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

impl std::ops::IndexMut<Register> for VmRegister {
    fn index_mut(&mut self, idx: Register) -> &mut Value {
        match idx {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
        }
    }
}
