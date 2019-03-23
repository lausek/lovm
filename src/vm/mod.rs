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
    stack: Vec<VmRegister>,
    code_stack: Vec<Value>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            memory: VmMemory::new(),
            stack: Vec::with_capacity(VM_STACK_SIZE),
            code_stack: Vec::with_capacity(VM_STACK_SIZE),
        }
    }
}

impl Vm {
    pub fn run(&mut self, bl: &CodeBlock) -> VmResult {
        let bl = bl.as_slice();
        let len = bl.len();
        let mut ip = 0usize;

        self.push_frame(None);

        while ip < len {
            match bl[ip] {
                Code::Instruction(inx) => {
                    println!("{:?}", inx);
                    match inx {
                        Instruction::Mov => {
                            let args = take(bl, &mut ip, 2);
                            let val = *read(&self, &args[1]);
                            write(self, &args[0], val);
                        }
                        Instruction::Inc | Instruction::Dec => {
                            let args = take(bl, &mut ip, 1);
                            let val = *read(&self, &args[0]);
                            match inx {
                                Instruction::Inc => write(self, &args[0], val + Value::U(1)),
                                Instruction::Dec => write(self, &args[0], val - Value::U(1)),
                                _ => unreachable!(),
                            }
                        }
                        Instruction::Add
                        | Instruction::Sub
                        | Instruction::Mul
                        | Instruction::Div
                        | Instruction::And
                        | Instruction::Or
                        | Instruction::Xor => {
                            let args = take(bl, &mut ip, 2);
                            let op1 = *read(&self, &args[0]);
                            let op2 = *read(&self, &args[1]);
                            println!("{:?}, {:?}", op1, op2);

                            let val = match inx {
                                Instruction::Add => op1 + op2,
                                Instruction::Sub => op1 - op2,
                                Instruction::Mul => op1 * op2,
                                Instruction::Div => op1 / op2,
                                Instruction::And => op1 & op2,
                                Instruction::Or => op1 | op2,
                                Instruction::Xor => op1 ^ op2,
                                _ => unimplemented!(),
                            };

                            write(self, &args[0], val)
                        }
                        Instruction::Cmp => {
                            let args = take(bl, &mut ip, 2);
                            let op1 = *read(&self, &args[0]);
                            let op2 = *read(&self, &args[1]);
                            (*register_mut(self)).cmp = op1.partial_cmp(&op2);
                        }
                        Instruction::Jmp
                        | Instruction::Jeq
                        | Instruction::Jne
                        | Instruction::Jge
                        | Instruction::Jgt
                        | Instruction::Jle
                        | Instruction::Jlt => {
                            let args = take(bl, &mut ip, 1);

                            if register(self).is_jmp_needed(inx) {
                                ip = usize::from(*read(&self, &args[0]));
                            } else {
                                ip += 1;
                            }

                            continue;
                        }
                        Instruction::Call => {
                            self.push_frame(Some(ip + 1));
                            let args = take(bl, &mut ip, 1);
                            ip = usize::from(*read(&self, &args[0]));
                        }
                        Instruction::Push => {
                            let args = take(bl, &mut ip, 1);
                            let val = *read(self, &args[0]);
                            self.code_stack.push(val);
                        }
                        Instruction::Pop => {
                            let val = self.code_stack.pop().expect("nothing to pop");
                            let args = take(bl, &mut ip, 1);
                            write(self, &args[0], val);
                        }
                        Instruction::Pusha => self.push_frame(None),
                        Instruction::Popa => self.pop_frame(None),
                        _ => println!("not implemented: `{:?}`", inx),
                    }
                }
                what => panic!("shall not happen! {:?}", what),
            }

            println!("regs: {:?}", register(self));

            ip += 1;
        }

        Ok(())
    }

    fn push_frame(&mut self, ret: Option<usize>) {
        if self.stack.is_empty() {
            self.stack.push(VmRegister::new());
        }
        let mut frame = register(self).clone();
        frame.ret = ret;
        self.stack.push(frame);
        *register_mut(self) = VmRegister::new();
    }

    fn pop_frame(&mut self, ip: Option<&mut usize>) {
        let frame = self.stack.pop().expect("frame to pop");
        match (ip, frame.ret) {
            (Some(ip), Some(jump_ip)) => *ip = jump_ip,
            _ => {}
        }
        *register_mut(self) = *self.stack.last().expect("no last frame");
    }
}

fn write(vm: &mut Vm, code: &'_ Code, value: Value) {
    match code {
        Code::Register(reg) => register_mut(vm)[*reg] = value,
        Code::Ref(addr) => vm.memory[*addr] = value,
        _ => unimplemented!(),
    };
}

fn read<'read, 'vm: 'read>(vm: &'vm Vm, code: &'read Code) -> &'read Value {
    match code {
        Code::Register(reg) => &register(vm)[*reg],
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

fn register(vm: &Vm) -> &VmRegister {
    vm.stack.last().expect("no last frame")
}

fn register_mut(vm: &mut Vm) -> &mut VmRegister {
    vm.stack.last_mut().expect("no last frame")
}
