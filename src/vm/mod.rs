pub mod memory;
pub mod operation;
pub mod register;

use crate::code::*;
use crate::value::*;

use self::memory::*;
use self::register::*;

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
    pub fn run(&mut self, program: &Program) -> VmResult {
        let bl = &program.codeblock;
        self.memory.map(bl, 0);

        let len = bl.len();
        let mut ip = program.entry_point().unwrap_or(0);

        self.push_frame(None);

        while ip < len {
            match self.memory[ip] {
                Code::Instruction(inx) => {
                    println!("{:?}", inx);
                    match inx {
                        Instruction::Mov
                        | Instruction::Load
                        | Instruction::Store
                        | Instruction::Copy => {
                            let args = take(bl, &mut ip, 2);
                            let val = match inx {
                                Instruction::Load | Instruction::Copy => {
                                    if let Code::Register(reg) = args[1] {
                                        let addr = register(self)[reg];
                                        *read(&self, &Code::Value(addr))
                                    } else {
                                        panic!("bytecode is invalid")
                                    }
                                }
                                _ => *read(&self, &args[1]),
                            };
                            let dest = match inx {
                                Instruction::Store | Instruction::Copy => {
                                    if let Code::Register(reg) = args[0] {
                                        Code::Value(register(self)[reg])
                                    } else {
                                        panic!("bytecode is invalid")
                                    }
                                }
                                _ => args[0],
                            };
                            write(self, &dest, val);
                        }
                        Instruction::Coal => {
                            let args = take(bl, &mut ip, 2);
                            let val = *read(&self, &args[0]);
                            let ty_idx = usize::from(*read(&self, &args[1]));
                            write(self, &args[0], val.cast(&Value::from_type(ty_idx)));
                        }
                        Instruction::Inc | Instruction::Dec => {
                            let args = take(bl, &mut ip, 1);
                            let val = *read(&self, &args[0]);
                            match inx {
                                Instruction::Inc => write(self, &args[0], val + Value::I(1)),
                                Instruction::Dec => write(self, &args[0], val - Value::I(1)),
                                _ => unreachable!(),
                            }
                        }
                        Instruction::Add
                        | Instruction::Sub
                        | Instruction::Mul
                        | Instruction::Div
                        | Instruction::Rem
                        | Instruction::Pow
                        | Instruction::Neg
                        | Instruction::And
                        | Instruction::Or
                        | Instruction::Xor
                        | Instruction::Shl
                        | Instruction::Shr => {
                            let args = take(bl, &mut ip, 2);
                            let op1 = *read(&self, &args[0]);
                            let op2 = *read(&self, &args[1]);
                            println!("{:?}, {:?}", op1, op2);

                            let val = match inx {
                                Instruction::Add => op1 + op2,
                                Instruction::Sub => op1 - op2,
                                Instruction::Mul => op1 * op2,
                                Instruction::Div => op1 / op2,
                                Instruction::Rem => op1 % op2,
                                Instruction::Pow => op1.pow(&op2),
                                Instruction::Neg => -op1,
                                Instruction::And => op1 & op2,
                                Instruction::Or => op1 | op2,
                                Instruction::Xor => op1 ^ op2,
                                Instruction::Shl => op1 << op2,
                                Instruction::Shr => op1 >> op2,
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
                                match &args[0] {
                                    Code::Value(Value::Ref(r)) => ip = *r,
                                    _ => panic!("invalid jump operand"),
                                }
                            } else {
                                ip += 1;
                            }

                            continue;
                        }
                        Instruction::Call => {
                            self.push_frame(Some(ip + 1));
                            let args = take(bl, &mut ip, 1);
                            match &args[0] {
                                Code::Value(Value::Ref(r)) => ip = *r,
                                _ => panic!("invalid jump operand"),
                            }
                            continue;
                        }
                        Instruction::Ret => self.pop_frame(Some(&mut ip)),
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
        self.stack.push(VmRegister::new());
        register_mut(self).ret = ret;
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
        Code::Value(vaddr) => {
            let addr = usize::from(*vaddr);
            vm.memory[addr] = Code::Value(value);
        }
        // TODO: reactivate this once typing is more efficient
        //Code::Value(Value::Ref(addr)) => vm.memory[*addr] = Code::Value(value),
        _ => unimplemented!(),
    };
}

fn read<'read, 'vm: 'read>(vm: &'vm Vm, code: &'read Code) -> &'read Value {
    match code {
        Code::Register(reg) => &register(vm)[*reg],
        Code::Value(Value::Ref(addr)) => match &vm.memory[*addr] {
            Code::Value(value) => value,
            code => panic!("unreadable memory accessed: {:?}", code),
        },
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
