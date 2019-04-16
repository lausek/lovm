use super::*;

use std::cmp;

#[derive(Clone, Copy, Debug)]
pub struct VmRegister {
    pub a: Value,
    pub b: Value,
    pub c: Value,
    pub d: Value,
    pub cmp: Option<cmp::Ordering>,
    pub ret: Option<usize>,
}

impl VmRegister {
    pub fn new() -> Self {
        Self {
            a: Value::I(0),
            b: Value::I(0),
            c: Value::I(0),
            d: Value::I(0),
            cmp: None,
            ret: None,
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
            Instruction::Jmp => true,
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
