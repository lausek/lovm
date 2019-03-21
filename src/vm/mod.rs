pub mod memory;
pub mod register;

use memory::*;
use register::*;

use crate::code::*;
use crate::value::*;

use std::cmp;

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
                            let val = *read(&self, &args[1]);
                            write(self, &args[0], val);
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
                        Instruction::Cmp => {
                            let args = take(bl, &mut ip, 2);
                            let op1 = *read(&self, &args[0]);
                            let op2 = *read(&self, &args[1]);
                            println!("{:?}, {:?}", op1, op2);

                            self.register.cmp = op1.partial_cmp(&op2);
                        }
                        Instruction::Jeq
                        | Instruction::Jne
                        | Instruction::Jge
                        | Instruction::Jgt
                        | Instruction::Jle
                        | Instruction::Jlt => {
                            let args = take(bl, &mut ip, 1);
                            let addr = usize::from(*read(&self, &args[0]));
                            let cmp = self.register.cmp.expect("no comparison");

                            match inx {
                                Instruction::Jeq if cmp == cmp::Ordering::Equal => ip = addr,
                                Instruction::Jne if cmp != cmp::Ordering::Equal => ip = addr,
                                Instruction::Jge
                                    if (cmp == cmp::Ordering::Greater)
                                        | (cmp == cmp::Ordering::Equal) =>
                                {
                                    ip = addr
                                }
                                Instruction::Jgt if cmp == cmp::Ordering::Greater => ip = addr,
                                Instruction::Jle
                                    if (cmp == cmp::Ordering::Less)
                                        | (cmp == cmp::Ordering::Equal) =>
                                {
                                    ip = addr
                                }
                                Instruction::Jlt if cmp == cmp::Ordering::Less => ip = addr,
                                // no jump will be executed
                                _ => {}
                            }

                            continue;
                        }
                        _ => println!("not implemented: `{:?}`", inx),
                    }
                }
                what => panic!("shall not happen! {:?}", what),
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
