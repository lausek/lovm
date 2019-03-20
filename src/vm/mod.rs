pub mod register;

use crate::code::*;
use crate::value::*;

pub const VM_MEMORY_SIZE: usize = 65556;
pub const VM_STACK_SIZE: usize = 256;

pub type VmResult = Result<(), String>;

pub struct Vm {
    memory: [Value; VM_MEMORY_SIZE],
    stack: [Value; VM_STACK_SIZE],
}

impl Vm {
    pub fn new() -> Self {
        Self {
            memory: [Value::U(0); VM_MEMORY_SIZE],
            stack: [Value::U(0); VM_STACK_SIZE],
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
