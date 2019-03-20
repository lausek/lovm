pub mod memory;
pub mod register;

use memory::*;

use crate::code::*;
use crate::value::*;

pub const VM_MEMORY_SIZE: usize = 2400;
pub const VM_STACK_SIZE: usize = 256;

pub type VmResult = Result<(), String>;

pub struct Vm {
    memory: VmMemory,
    stack: [Value; VM_STACK_SIZE],
}

impl Vm {
    pub fn new() -> Self {
        Self {
            memory: VmMemory::new(),
            stack: [Value::U(0); VM_STACK_SIZE],
        }
    }
}

impl Vm {
    pub fn run(&mut self, bl: &CodeBlock) -> VmResult {
        let bl = bl.as_slice();
        let len = bl.len();
        let mut ip = 0usize;

        while ip < len {
            match bl[ip] {
                Code::Instruction(inx) => println!("{:?}", inx),
                Code::Value(value) => println!("{:?}", value),
            }

            ip += 1;
        }

        Ok(())
    }
}
