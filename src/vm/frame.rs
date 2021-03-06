use super::*;

#[derive(Clone, Debug)]
pub struct VmFrame {
    pub locals: Vec<Value>,
}

// TODO: add new constructor with preinitialized array
impl VmFrame {
    pub fn new(argc: usize) -> Self {
        Self {
            locals: (0..argc).map(|_| Value::I(0)).collect(),
        }
    }
}
