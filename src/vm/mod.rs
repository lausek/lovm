pub mod memory;
pub mod register;

use memory::*;
use register::*;

use crate::code::*;
use crate::value::*;

pub const VM_MEMORY_SIZE: usize = 2400;
pub const VM_STACK_SIZE: usize = 256;

pub type VmResult = Result<(), String>;

pub struct Vm {
    memory: VmMemory,
    register: VmRegister,
    stack: [Value; VM_STACK_SIZE],
}

impl Vm {
    pub fn new() -> Self {
        Self {
            memory: VmMemory::new(),
            register: VmRegister::new(),
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
                Code::Instruction(inx) => {
                    println!("{:?}", inx);
                    match inx {
                        Instruction::Store => {
                            let args = take(bl, &mut ip, 2);
                            let val = match args[1] {
                                Code::Register(reg) => self.register[reg],
                                Code::Ref(addr) => self.memory[addr],
                                Code::Value(value) => value,
                                _ => unimplemented!(),
                            };
                            match args[0] {
                                Code::Register(reg) => self.register[reg] = val,
                                Code::Ref(addr) => self.memory[addr] = val,
                                _ => unreachable!(),
                            }
                            println!("first arg {:?}", args[0]);
                            println!("second arg {:?}", args[1]);
                        }
                        Instruction::Add
                        | Instruction::Sub
                        | Instruction::Mul
                        | Instruction::Div => {
                            let args = take(bl, &mut ip, 2);
                            let op1 = *read(&self, &args[0]);
                            let op2 = *read(&self, &args[1]);
                            println!("{:?}, {:?}", op1, op2);

                            let val = match inx {
                                Instruction::Add => op1 + op2,
                                Instruction::Sub => op1 - op2,
                                Instruction::Mul => op1 * op2,
                                Instruction::Div => op1 / op2,
                                _ => unimplemented!(),
                            };

                            write(self, &args[0], val)
                        }
                        _ => println!("not implemented: `{:?}`", inx),
                    }
                }
                _ => panic!("shall not happen!"),
            }

            println!("regs: {:?}", self.register);

            ip += 1;
        }

        Ok(())
    }
}

fn write(vm: &mut Vm, code: &'_ Code, value: Value) {
    match code {
        Code::Register(reg) => vm.register[*reg] = value,
        Code::Ref(addr) => vm.memory[*addr] = value,
        _ => unimplemented!(),
    };
}

fn read<'read, 'vm: 'read>(vm: &'vm Vm, code: &'read Code) -> &'read Value {
    match code {
        Code::Register(reg) => &vm.register[*reg],
        Code::Ref(addr) => &vm.memory[*addr],
        Code::Value(value) => value,
        _ => unimplemented!(),
    }
}

fn take<'bl>(bl: &'bl [Code], ip: &mut usize, n: usize) -> &'bl [Code] {
    let view = &bl[*ip + 1..=*ip + n];
    *ip += n;
    view
}
