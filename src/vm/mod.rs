pub mod register;

use crate::code::*;

pub const VM_MEMORY_SIZE: usize = 65556;
pub const VM_STACK_SIZE: usize = 255;

pub type VmResult = Result<(), String>;

pub struct Vm {
    memory: [Code; VM_MEMORY_SIZE],
    stack: [Code; VM_STACK_SIZE],
}

impl Vm {
    pub fn new() -> Self {
        Self {
            memory: [Code::Instruction(0x0); VM_MEMORY_SIZE],
            stack: [Code::Instruction(0x0); VM_STACK_SIZE],
        }
    }
}

impl Vm {
    pub fn run(bl: &CodeBlock) -> VmResult {
        let bl = bl.as_slice();
        let len = bl.len();
        let mut ip = 0usize;

        while ip < len {
            match bl[ip] {
                Code::Instruction(_) => println!(""),
                Code::Value(_) => println!(""),
            }

            ip += 1;
        }

        Ok(())
    }
}
