pub mod frame;
pub mod interrupt;
pub mod module;
pub mod object;
pub mod operation;

use super::*;

pub use self::frame::*;
pub use self::interrupt::*;
pub use self::module::*;
pub use self::object::*;

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
    Panic,
    Exited,
}

// TODO: add `lru_cache` (least-recently used optimization) to
// improve runtime speed e.g. for `fib`
#[derive(Clone, Debug)]
pub struct VmData {
    pub globals: HashMap<Name, Value>,
    pub modules: Units,
    pub obj_pool: ObjectPool,
    pub state: VmState,
    pub stack: Vec<VmFrame>,
    pub vstack: Vec<Value>,
}

impl VmData {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            modules: Units::new(),
            obj_pool: ObjectPool::new(),
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
    fn panic<T, V>(&mut self, msg: T) -> Result<V, String>
    where
        T: Into<String>,
    {
        self.data.state = VmState::Panic;
        Err(msg.into())
    }

    // TODO: return `Rc` over `CodeObject` here because it could reassign itself
    fn call_lookup(&self, name: &Name) -> Result<&CodeObject, String> {
        match self.data.modules.lookup(name) {
            Some(item) => Ok(item),
            _ => Err(format!("function `{}` is unknown", name)),
        }
    }

    pub fn run_object(&mut self, co: &CodeObject) -> VmResult {
        let bl = &co.inner;
        let len = bl.len();
        let mut ip = 0;

        self.push_frame(co.space.locals.len());

        while self.data.state == VmState::Running && ip < len {
            let inx = &bl[ip];

            if cfg!(debug_assertions) {
                println!(
                    "{}: {:?} {}",
                    ip,
                    inx,
                    inx.arg().map_or("".to_string(), |arg| match inx {
                        Instruction::CPush(_) => format!(":= {}", co.space.consts[arg]),
                        Instruction::LPush(_) | Instruction::LPop(_) => {
                            format!(":= {}", co.space.locals[arg])
                        }
                        Instruction::GPush(_) | Instruction::GPop(_) | Instruction::GCall(_) => {
                            format!(":= {}", co.space.globals[arg])
                        }
                        _ => "".to_string(),
                    })
                );
            }

            match inx {
                // ret is needed for early returns
                Instruction::Ret => break,
                Instruction::Pusha => self.push_frame(0),
                Instruction::Popa => self.pop_frame(),
                Instruction::Dup => {
                    let dup = self.data.vstack.last().expect("no value").clone();
                    self.data.vstack.push(dup);
                }
                Instruction::Int(idx) => {
                    if let Some(irh) = self.interrupts.get(*idx) {
                        irh(&mut self.data)?;
                    }
                }
                Instruction::Cast(ty_idx) => {
                    let val = self.data.vstack.last_mut().expect("no value");
                    *val = val.cast(&Value::from_type(*ty_idx));
                }
                Instruction::LPop(idx) | Instruction::GPop(idx) => {
                    let value = self.data.vstack.pop().expect("no value");
                    match inx {
                        Instruction::LPop(_) => {
                            frame_mut(&mut self.data).locals[*idx] = value;
                        }
                        Instruction::GPop(_) => {
                            let name = co.space.globals.get(*idx).unwrap();
                            self.data.globals.insert(name.clone(), value);
                        }
                        _ => unreachable!(),
                    }
                }
                Instruction::CPush(idx) | Instruction::LPush(idx) | Instruction::GPush(idx) => {
                    let value = match inx {
                        Instruction::CPush(_) => co.space.consts[*idx].clone(),
                        Instruction::LPush(_) => frame(&self.data).locals[*idx].clone(),
                        Instruction::GPush(_) => {
                            let name = co.space.globals.get(*idx).unwrap();
                            match self.data.globals.get(name) {
                                Some(value) => value.clone(),
                                _ => self.panic(format!("`{}` was not declared", name))?,
                            }
                        }
                        _ => unreachable!(),
                    };
                    self.data.vstack.push(value);
                }
                Instruction::LCall(_idx) => {
                    unimplemented!();
                }
                Instruction::GCall(idx) => {
                    let fname = &co.space.globals[*idx];
                    let co = self.call_lookup(&fname.to_string())?.clone();
                    self.run_object(&co)?;
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
                Instruction::Neg => {
                    let target = self.data.vstack.last_mut().expect("no target");
                    *target = target.neg();
                }
                Instruction::Add
                | Instruction::Sub
                | Instruction::Mul
                | Instruction::Div
                | Instruction::Rem
                | Instruction::Pow
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
                        Instruction::And => target.and(&op),
                        Instruction::Or => target.or(&op),
                        Instruction::Xor => target.xor(&op),
                        Instruction::Shl => target.shl(&op),
                        Instruction::Shr => target.shr(&op),
                        _ => unimplemented!(),
                    };
                }
                Instruction::CmpEq
                | Instruction::CmpNe
                | Instruction::CmpGe
                | Instruction::CmpGt
                | Instruction::CmpLe
                | Instruction::CmpLt => {
                    use std::cmp::Ordering;
                    let op1 = self.data.vstack.pop().expect("missing op1");
                    let op2 = self.data.vstack.pop().expect("missing op2");
                    let inx = *inx;
                    let cond = match op2.partial_cmp(&op1).unwrap() {
                        Ordering::Equal => {
                            inx == Instruction::CmpEq
                                || inx == Instruction::CmpGe
                                || inx == Instruction::CmpLe
                        }
                        Ordering::Greater => {
                            inx == Instruction::CmpNe
                                || inx == Instruction::CmpGe
                                || inx == Instruction::CmpGt
                        }
                        Ordering::Less => {
                            inx == Instruction::CmpNe
                                || inx == Instruction::CmpLe
                                || inx == Instruction::CmpLt
                        }
                    };
                    self.data.vstack.push(Value::T(cond));
                }
                Instruction::Jmp(nip) => {
                    ip = *nip;
                    continue;
                }
                Instruction::Jt(nip) | Instruction::Jf(nip) => {
                    let cond: bool = self.data.vstack.pop().expect("no condition").into();
                    if match inx {
                        Instruction::Jt(_) => cond,
                        Instruction::Jf(_) => !cond,
                        _ => unreachable!(),
                    } {
                        ip = *nip;
                        continue;
                    }
                }
                Instruction::ONew => {
                    let handle = self.data.obj_pool.new_handle();
                    self.data.vstack.push(Value::Ref(handle));
                }
                Instruction::ONewDict => {
                    let handle = self.data.obj_pool.new_dict_handle();
                    self.data.vstack.push(Value::Ref(handle));
                }
                Instruction::ONewArray => {
                    let handle = self.data.obj_pool.new_array_handle();
                    self.data.vstack.push(Value::Ref(handle));
                }
                Instruction::ODispose => {
                    let handle = usize::from(self.data.vstack.pop().expect("no object"));
                    self.data.obj_pool.dispose_handle(&handle);
                }
                Instruction::OCall(idx) => {
                    let aname = &co.space.consts[*idx];
                    let cb = {
                        let object = object(&self.data);
                        object.lookup(aname).expect("no method found")
                    };
                    // TODO: check if locals[0] is self => assign self = vstack.last()
                    // TODO: ugh... remove this clone pls
                    self.run_object(&cb.clone())?;
                }
                Instruction::OAppend => {
                    let value = self.data.vstack.pop().expect("no value");
                    object_mut(&mut self.data)
                        .as_indexable()
                        .unwrap()
                        .append(value);
                }
                Instruction::OGet(idx) => {
                    let aname = &co.space.consts[*idx];
                    let value = {
                        let object = object_mut(&mut self.data).as_indexable().unwrap();
                        object.getk(&aname).expect("unknown attribute").clone()
                    };
                    self.data.vstack.push(value);
                }
                Instruction::OSet(idx) => {
                    let value = self.data.vstack.pop().expect("no value");
                    let object = object_mut(&mut self.data).as_indexable().unwrap();
                    let aname = &co.space.consts[*idx];
                    object.setk(&aname, value);
                }
            }

            if cfg!(debug_assertions) {
                println!("{:?}", self.data.vstack);
            }

            ip += 1;
        }

        self.pop_frame();

        Ok(())
    }

    pub fn run(&mut self, module: &Unit) -> VmResult {
        // loads the programs main function
        let co = &module.code();

        // TODO: something better than cloning?
        self.data.modules.load(module)?;
        self.data.state = VmState::Running;
        self.run_object(co)
    }

    fn push_frame(&mut self, argc: usize) {
        self.data.stack.push(VmFrame::new(argc));
    }

    fn pop_frame(&mut self) {
        let _last = self.data.stack.pop().expect("frame to pop");
        if cfg!(debug_assertions) {
            println!("last frame {:?}", _last);
        }

        if self.data.stack.is_empty() {
            self.data.state = VmState::Exited;
        } else {
            *frame_mut(&mut self.data) = self.data.stack.last().expect("no last frame").clone();
        }
    }
}

fn object(vm: &VmData) -> &Object {
    match vm.vstack.last().expect("no object ref") {
        Value::Ref(handle) => vm.obj_pool.get(&handle).unwrap(),
        _ => unimplemented!(),
    }
}

fn object_mut(vm: &mut VmData) -> &mut Object {
    match vm.vstack.last().expect("no object ref") {
        Value::Ref(handle) => vm.obj_pool.get_mut(&handle).unwrap(),
        _ => unimplemented!(),
    }
}

fn frame(vm: &VmData) -> &VmFrame {
    vm.stack.last().expect("no last frame")
}

fn frame_mut(vm: &mut VmData) -> &mut VmFrame {
    vm.stack.last_mut().expect("no last frame")
}
