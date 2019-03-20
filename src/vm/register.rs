use super::*;

#[derive(Clone, Copy, Debug)]
pub struct VmRegister {
    a: Value,
    b: Value,
    c: Value,
    d: Value,
}

impl VmRegister {
    pub fn new() -> Self {
        Self {
            a: Value::U(0),
            b: Value::U(0),
            c: Value::U(0),
            d: Value::U(0),
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
