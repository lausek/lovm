use super::*;

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum Interrupt {
    Dbg = 10,
}

pub type InterruptHandler = fn(&mut Vm) -> VmResult;

pub struct Interrupts([Option<InterruptHandler>; 256]);

impl Interrupts {
    pub fn new() -> Self {
        Self([None; 256])
    }

    pub fn get(&self, idx: usize) -> Option<&InterruptHandler> {
        self.0.get(idx).unwrap().as_ref()
    }

    pub fn set(&mut self, idx: usize, ir: Option<InterruptHandler>) {
        *self.0.get_mut(idx).unwrap() = ir;
    }
}
