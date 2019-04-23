pub mod frame;
pub mod interrupt;
pub mod operation;
pub mod str;

use super::*;

use self::frame::*;
use self::interrupt::*;
use self::str::*;

pub use std::collections::HashMap;

// the vm is meant to be used as a dynamic runtime. it keeps track of:
//  - globals: area for storing global vm values
//  - modules: loaded vm modules; used for name lookup (e.g. in function call)
//  - obj_pool: all allocated custom objects
//  - state: status flag for vm flow control
//  - stack: callstack consisting of local frames
//  - vstack: global value stack; used for returning values (?)
//
// INFO: see operation.rs for more

// TODO: rename `vm` to `runtime` to avoid name conflicts with vm binary (?)

pub const VM_MEMORY_SIZE: usize = 2400;
pub const VM_STACK_SIZE: usize = 256;

pub type VmResult = Result<(), String>;

#[derive(PartialEq)]
pub enum VmState {
    Initial,
    Running,
    Exited,
}

pub struct VmData {
    pub globals: HashMap<Name, Value>,
    pub modules: Vec<Module>,
    pub obj_pool: HashMap<Name, ()>,
    pub state: VmState,
    pub stack: Vec<VmFrame>,
    pub vstack: Vec<Value>,
}

impl VmData {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            modules: vec![],
            obj_pool: HashMap::new(),
            state: VmState::Initial,
            stack: vec![],
            vstack: vec![],
        }
    }
}

pub struct Vm {
    interrupts: Interrupts,
    pub data: VmData,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            interrupts: Interrupts::default(),
            data: VmData::new(),
        }
    }

    pub fn interrupts_mut(&mut self) -> &mut Interrupts {
        &mut self.interrupts
    }
}

impl Vm {
    fn run_frame(&mut self, co: &CodeObject) -> VmResult {
        // loads the programs main function
        let bl = &co.inner;
        let len = bl.len();
        let mut ip = 0;

        self.push_frame(None);
        self.data.state = VmState::Running;

        while self.data.state == VmState::Running && ip < len {
            match &bl[ip] {
                Code::Instruction(inx) => {
                    if cfg!(debug_assertions) {
                        println!("{}: {:?}", ip, inx);
                    }

                    if inx == &Instruction::Call {
                        self.push_frame(Some(ip + 1));
                    }

                    let argc = inx.arguments();
                    let args = take(bl, &mut ip, argc);

                    match inx {
                        Instruction::Load => {
                            let val = self.data.vstack.pop().expect("missing address");
                            self.data.vstack.push(read_memory(&self, &val).clone());
                        }
                        Instruction::Store => {
                            let addr = self.data.vstack.pop().expect("missing address");
                            let val = self.data.vstack.pop().expect("missing value");
                            write(self, &Code::Value(addr), val);
                        }
                        Instruction::Int => {
                            let idx = usize::from(read(&self, &args[0]).clone());
                            if let Some(irh) = self.interrupts.get(idx) {
                                irh(&mut self.data)?;
                            } else {
                                return Err(format!("interrupt {} not defined", idx));
                            }
                        }
                        Instruction::Cast => {
                            let ty_idx = usize::from(read(&self, &args[0]).clone());
                            let val = self.data.vstack.last_mut().expect("no value");
                            *val = val.cast(&Value::from_type(ty_idx));
                        }
                        Instruction::Inc | Instruction::Dec => {
                            let val = read(&self, &args[0]);
                            match inx {
                                Instruction::Inc => write(self, &args[0], val.add(&Value::I(1))),
                                Instruction::Dec => write(self, &args[0], val.sub(&Value::I(1))),
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
                            let op2 = self.data.vstack.pop().expect("no operand");
                            let op1 = self.data.vstack.last_mut().expect("no target");
                            if cfg!(debug_assertions) {
                                println!("{:?}, {:?}", op1, op2);
                            }

                            // TODO: deref causes copy when inplace modification would be enough
                            let val = match inx {
                                Instruction::Add => op1.add(&op2),
                                Instruction::Sub => op1.sub(&op2),
                                Instruction::Mul => op1.mul(&op2),
                                Instruction::Div => op1.div(&op2),
                                Instruction::Rem => op1.rem(&op2),
                                Instruction::Pow => op1.pow(&op2),
                                Instruction::Neg => op1.neg(),
                                Instruction::And => op1.and(&op2),
                                Instruction::Or => op1.or(&op2),
                                Instruction::Xor => op1.xor(&op2),
                                Instruction::Shl => op1.shl(&op2),
                                Instruction::Shr => op1.shr(&op2),
                                _ => unimplemented!(),
                            };

                            *op1 = val;
                        }
                        Instruction::Cmp => {
                            let op2 = self.data.vstack.pop().expect("missing op2");
                            let op1 = self.data.vstack.pop().expect("missing op1");
                            (*register_mut(&mut self.data)).cmp = op1.partial_cmp(&op2);
                        }
                        Instruction::Jmp
                        | Instruction::Jeq
                        | Instruction::Jne
                        | Instruction::Jge
                        | Instruction::Jgt
                        | Instruction::Jle
                        | Instruction::Jlt => {
                            if register(&self.data).is_jmp_needed(&inx) {
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
                            // TODO: lookup the name on the stack
                            // TODO: call `run` again with new `CodeObject`
                            match &args[0] {
                                Code::Value(Value::Ref(r)) => ip = *r,
                                _ => panic!("invalid jump operand"),
                            }
                            continue;
                        }
                        Instruction::Ret => self.pop_frame(Some(&mut ip)),
                        Instruction::Push => {
                            let val = read(self, &args[0]);
                            self.data.vstack.push(val.clone());
                        }
                        Instruction::Pop => {
                            let val = self.data.vstack.pop().expect("nothing to pop");
                            write(self, &args[0], val);
                        }
                        Instruction::Pusha => self.push_frame(None),
                        Instruction::Popa => self.pop_frame(None),
                    }
                }
                what => panic!("non-executable code reached {:?}", what),
            }

            if cfg!(debug_assertions) {
                println!("{:?}", self.data.vstack);
            }

            ip += 1;
        }
        Ok(())
    }

    pub fn run(&mut self, module: &Module) -> VmResult {
        let co = &module.code();
        self.run_frame(co)
    }

    fn push_frame(&mut self, ret: Option<usize>) {
        self.data.stack.push(VmFrame::new());
        register_mut(&mut self.data).ret = ret;
    }

    fn pop_frame(&mut self, ip: Option<&mut usize>) {
        let frame = self.data.stack.pop().expect("frame to pop");

        if let (Some(ip), Some(jump_ip)) = (ip, frame.ret) {
            *ip = jump_ip;
        }

        if self.data.stack.is_empty() {
            self.data.state = VmState::Exited;
        } else {
            *register_mut(&mut self.data) = self.data.stack.last().expect("no last frame").clone();
        }
    }
}

fn write(vm: &mut Vm, code: &'_ Code, value: Value) {
    match code {
        Code::Register(reg) => register_mut(&mut vm.data)[*reg] = value,
        Code::Value(vaddr) => {
            unimplemented!()
            //let addr = usize::from(*vaddr);
            //vm.data.memory[addr] = Code::Value(value);
        }
        // TODO: reactivate this once typing is more efficient
        //Code::Value(Value::Ref(addr)) => vm.memory[*addr] = Code::Value(value),
        _ => unimplemented!(),
    };
}

fn read<'read, 'vm: 'read>(vm: &'vm Vm, code: &'read Code) -> &'read Value {
    match code {
        Code::Register(reg) => &register(&vm.data)[*reg],
        Code::Value(value) => value,
        _ => unimplemented!(),
    }
}

fn read_memory<'read, 'vm: 'read>(vm: &'vm Vm, code: &'read Value) -> &'read Value {
    unimplemented!()
    //let addr = usize::from(*code);
    //match &vm.data.memory[addr] {
    //    Code::Value(value) => &value,
    //    code => panic!("unreadable memory accessed: {:?}, addr {}", code, addr),
    //}
}

fn take<'bl>(bl: &'bl [Code], ip: &mut usize, n: usize) -> &'bl [Code] {
    let view = &bl[*ip + 1..=*ip + n];
    *ip += n;
    view
}

fn register(vm: &VmData) -> &VmFrame {
    vm.stack.last().expect("no last frame")
}

fn register_mut(vm: &mut VmData) -> &mut VmFrame {
    vm.stack.last_mut().expect("no last frame")
}
