pub mod frame;
pub mod interrupt;
pub mod operation;

use super::*;

pub use self::frame::*;
pub use self::interrupt::*;

pub use std::collections::HashMap;

// the vm is meant to be used as a dynamic runtime. it keeps track of:
//  - globals: area for storing global vm values
//  - modules: loaded vm modules; used for name lookup (e.g. in function call)
//  - obj_pool: all allocated custom objects
//  - state: status flag for vm flow control
//  - stack: callstack consisting of local frames
//  - vstack: global value stack; used for returning values (?)
//
// the register-based implementation approach was dropped in favor of stack-based
// processing because it can be implemented in a straight forward fashion without
// too many local indirections. however, it is possible that they might return
// later when the performance expectations are higher.
//
// INFO: see operation.rs for more

// TODO: rename `vm` to `runtime` to avoid name conflicts with vm binary (?)

pub const VM_MEMORY_SIZE: usize = 2400;
pub const VM_STACK_SIZE: usize = 256;

pub type VmResult = Result<(), String>;

#[derive(Clone, Debug, PartialEq)]
pub enum VmState {
    Initial,
    Running,
    Exited,
}

#[derive(Clone, Debug)]
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
    // TODO: return `Rc` over `CodeObject` here because it could reassign itself
    fn call_lookup(&self, name: &Name) -> Result<&CodeObject, String> {
        for module in self.data.modules.iter() {
            if let Some(co) = module.get(name) {
                return Ok(co);
            }
        }
        Err(format!("function `{}` is unknown", name))
    }

    pub fn run_object(&mut self, co: &CodeObject) -> VmResult {
        let bl = &co.inner;
        let len = bl.len();
        let mut ip = 0;

        self.push_frame(co.space.locals.len());

        for i in 0..co.argc {
            register_mut(&mut self.data).locals[i] = self.data.vstack.pop().expect("no argument");
        }

        while self.data.state == VmState::Running && ip < len {
            let inx = &bl[ip];

            if cfg!(debug_assertions) {
                println!("{}: {:?}", ip, inx);
            }

            match inx {
                // ret is needed for early returns
                Instruction::Ret => break,
                Instruction::Int(idx) => {
                    if let Some(irh) = self.interrupts.get(*idx) {
                        irh(&mut self.data)?;
                    } else {
                        return Err(format!("interrupt {} not defined", idx));
                    }
                }
                Instruction::Cast(ty_idx) => {
                    let val = self.data.vstack.last_mut().expect("no value");
                    *val = val.cast(&Value::from_type(*ty_idx));
                }
                Instruction::Call => {
                    let fname = self.data.vstack.pop().expect("no function name");
                    let co = self.call_lookup(&fname.to_string())?.clone();
                    self.run_object(&co)?;
                }
                Instruction::Lpop(idx) | Instruction::Gpop(idx) => {
                    let value = self.data.vstack.pop().expect("no value");
                    match inx {
                        Instruction::Lpop(_) => {
                            register_mut(&mut self.data).locals[*idx] = value;
                        }
                        Instruction::Gpop(_) => {
                            let name = co.space.globals.get(*idx).unwrap();
                            self.data.globals.insert(name.clone(), value);
                        }
                        _ => unreachable!(),
                    }
                }
                Instruction::Cpush(idx) | Instruction::Lpush(idx) | Instruction::Gpush(idx) => {
                    let value = match inx {
                        Instruction::Cpush(_) => co.space.consts[*idx].clone(),
                        Instruction::Lpush(_) => register(&self.data).locals[*idx].clone(),
                        Instruction::Gpush(_) => {
                            let name = co.space.globals.get(*idx).unwrap();
                            self.data.globals.get(name).unwrap().clone()
                        }
                        _ => unreachable!(),
                    };
                    self.data.vstack.push(value);
                }
                Instruction::Inc | Instruction::Dec => {
                    unimplemented!();
                    // `increment` and `decrement` are common operations and allow for
                    // inplace modifications instead of computation over the stack.
                    // TODO: implement inc and dec
                    //let val = read(&self, &args[0]);
                    //match inx {
                    //    Instruction::Inc => write(self, &args[0], val.add(&Value::I(1))),
                    //    Instruction::Dec => write(self, &args[0], val.sub(&Value::I(1))),
                    //    _ => unreachable!(),
                    //}
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
                    let op = self.data.vstack.pop().expect("no operand");
                    let target = self.data.vstack.last_mut().expect("no target");

                    if cfg!(debug_assertions) {
                        println!("target({:?}) {:?} {:?}", target, inx, op);
                    }

                    *target = match inx {
                        Instruction::Add => target.add(&op),
                        Instruction::Sub => target.sub(&op),
                        Instruction::Mul => target.mul(&op),
                        Instruction::Div => target.div(&op),
                        Instruction::Rem => target.rem(&op),
                        Instruction::Pow => target.pow(&op),
                        // TODO: Neg does not not an operand
                        Instruction::Neg => op.neg(),
                        Instruction::And => target.and(&op),
                        Instruction::Or => target.or(&op),
                        Instruction::Xor => target.xor(&op),
                        Instruction::Shl => target.shl(&op),
                        Instruction::Shr => target.shr(&op),
                        _ => unimplemented!(),
                    };
                }
                Instruction::Cmp => {
                    let op1 = self.data.vstack.pop().expect("missing op1");
                    let op2 = self.data.vstack.pop().expect("missing op2");
                    (*register_mut(&mut self.data)).cmp = op2.partial_cmp(&op1);
                }
                Instruction::Jmp(nip)
                | Instruction::Jeq(nip)
                | Instruction::Jne(nip)
                | Instruction::Jge(nip)
                | Instruction::Jgt(nip)
                | Instruction::Jle(nip)
                | Instruction::Jlt(nip) => {
                    if register(&self.data).is_jmp_needed(&inx) {
                        ip = *nip;
                        continue;
                    }
                }
                Instruction::Pusha => self.push_frame(0),
                Instruction::Popa => self.pop_frame(),
            }

            if cfg!(debug_assertions) {
                println!("{:?}", self.data.vstack);
            }

            ip += 1;
        }

        self.pop_frame();

        Ok(())
    }

    pub fn run(&mut self, module: &Module) -> VmResult {
        // loads the programs main function
        let co = &module.code();

        // TODO: something better than cloning?
        self.data.modules.push(module.clone());
        self.data.state = VmState::Running;
        self.run_object(co)
    }

    fn push_frame(&mut self, argc: usize) {
        self.data.stack.push(VmFrame::new(argc));
    }

    fn pop_frame(&mut self) {
        self.data.stack.pop().expect("frame to pop");

        if self.data.stack.is_empty() {
            self.data.state = VmState::Exited;
        } else {
            *register_mut(&mut self.data) = self.data.stack.last().expect("no last frame").clone();
        }
    }
}

fn register(vm: &VmData) -> &VmFrame {
    vm.stack.last().expect("no last frame")
}

fn register_mut(vm: &mut VmData) -> &mut VmFrame {
    vm.stack.last_mut().expect("no last frame")
}
