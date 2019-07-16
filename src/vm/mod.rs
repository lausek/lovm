pub mod frame;
pub mod interrupt;
pub mod object;
pub mod operation;
pub mod unit;

use super::*;

pub use self::frame::*;
pub use self::interrupt::*;
pub use self::object::*;
pub use self::unit::*;

pub use std::collections::HashMap;

// the vm is meant to be used as a dynamic runtime. it keeps track of:
//  - globals: area for storing global vm values
//  - units: loaded vm units; used for name lookup (e.g. in function call)
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
    pub units: Units,
    pub obj_pool: ObjectPool,
    pub state: VmState,
    pub stack: Vec<VmFrame>,
    pub vstack: Vec<Value>,
}

impl VmData {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            units: Units::new(),
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

    fn call_lookup(&self, name: &Name) -> Result<CodeObjectRef, String> {
        match self.data.units.lookup(name) {
            Some(item) => Ok(item),
            _ => Err(format!("function `{}` is unknown", name)),
        }
    }

    pub fn run_object(&mut self, co: CodeObjectRef) -> VmResult {
        let co: &CodeObject = co.borrow();
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
                        Code::CPush(_) => format!(":= {}", co.space.consts[arg]),
                        Code::LPush(_) | Code::LPop(_) => format!(":= {}", co.space.locals[arg]),
                        Code::GPush(_) | Code::GPop(_) | Code::GCall(_) => {
                            format!(":= {}", co.space.globals[arg])
                        }
                        _ => "".to_string(),
                    })
                );
            }

            match inx {
                // ret is needed for early returns
                Code::Ret => break,
                Code::Pusha => self.push_frame(0),
                Code::Popa => self.pop_frame(),
                Code::Dup => {
                    let dup = self.data.vstack.last().expect("no value").clone();
                    self.data.vstack.push(dup);
                }
                Code::Int(idx) => {
                    if let Some(irh) = self.interrupts.get(*idx) {
                        irh(&mut self.data)?;
                    }
                }
                Code::Cast(ty_idx) => {
                    let val = self.data.vstack.last_mut().expect("no value");
                    *val = val.cast(&Value::from_type(*ty_idx));
                }
                Code::LPop(idx) | Code::GPop(idx) => {
                    let value = self.data.vstack.pop().expect("no value");
                    match inx {
                        Code::LPop(_) => {
                            frame_mut(&mut self.data).locals[*idx] = value;
                        }
                        Code::GPop(_) => {
                            let name = co.space.globals.get(*idx).unwrap();
                            self.data.globals.insert(name.clone(), value);
                        }
                        _ => unreachable!(),
                    }
                }
                Code::CPush(idx) | Code::LPush(idx) | Code::GPush(idx) => {
                    let value = match inx {
                        Code::CPush(_) => co.space.consts[*idx].clone(),
                        Code::LPush(_) => frame(&self.data).locals[*idx].clone(),
                        Code::GPush(_) => {
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
                Code::LCall(_idx) => {
                    unimplemented!();
                }
                Code::GCall(idx) => {
                    let fname = &co.space.globals[*idx];
                    let co = self.call_lookup(&fname.to_string())?;
                    self.run_object(co)?;
                }
                Code::Inc | Code::Dec => {
                    unimplemented!();
                    // `increment` and `decrement` are common operations and allow for
                    // inplace modifications instead of computation over the stack.
                    // TODO: implement inc and dec
                    //let val = read(&self, &args[0]);
                    //match inx {
                    //    Code::Inc => write(self, &args[0], val.add(&Value::I(1))),
                    //    Code::Dec => write(self, &args[0], val.sub(&Value::I(1))),
                    //    _ => unreachable!(),
                    //}
                }
                Code::Neg => {
                    let target = self.data.vstack.last_mut().expect("no target");
                    *target = target.neg();
                }
                Code::Add
                | Code::Sub
                | Code::Mul
                | Code::Div
                | Code::Rem
                | Code::Pow
                | Code::And
                | Code::Or
                | Code::Xor
                | Code::Shl
                | Code::Shr => {
                    let op = self.data.vstack.pop().expect("no operand");
                    let target = self.data.vstack.last_mut().expect("no target");

                    if cfg!(debug_assertions) {
                        println!("target({:?}) {:?} {:?}", target, inx, op);
                    }

                    *target = match inx {
                        Code::Add => target.add(&op),
                        Code::Sub => target.sub(&op),
                        Code::Mul => target.mul(&op),
                        Code::Div => target.div(&op),
                        Code::Rem => target.rem(&op),
                        Code::Pow => target.pow(&op),
                        Code::And => target.and(&op),
                        Code::Or => target.or(&op),
                        Code::Xor => target.xor(&op),
                        Code::Shl => target.shl(&op),
                        Code::Shr => target.shr(&op),
                        _ => unimplemented!(),
                    };
                }
                Code::CmpEq
                | Code::CmpNe
                | Code::CmpGe
                | Code::CmpGt
                | Code::CmpLe
                | Code::CmpLt => {
                    use std::cmp::Ordering;
                    let op1 = self.data.vstack.pop().expect("missing op1");
                    let op2 = self.data.vstack.pop().expect("missing op2");
                    let inx = *inx;
                    let cond = match op2.partial_cmp(&op1).unwrap() {
                        Ordering::Equal => {
                            inx == Code::CmpEq || inx == Code::CmpGe || inx == Code::CmpLe
                        }
                        Ordering::Greater => {
                            inx == Code::CmpNe || inx == Code::CmpGe || inx == Code::CmpGt
                        }
                        Ordering::Less => {
                            inx == Code::CmpNe || inx == Code::CmpLe || inx == Code::CmpLt
                        }
                    };
                    self.data.vstack.push(Value::T(cond));
                }
                Code::Jmp(nip) => {
                    ip = *nip;
                    continue;
                }
                Code::Jt(nip) | Code::Jf(nip) => {
                    let cond: bool = self.data.vstack.pop().expect("no condition").into();
                    if match inx {
                        Code::Jt(_) => cond,
                        Code::Jf(_) => !cond,
                        _ => unreachable!(),
                    } {
                        ip = *nip;
                        continue;
                    }
                }
                Code::ONew(idx) => {
                    let ty = &co.space.globals[*idx];
                    let uref = self.data.units.lookup_ty(ty).expect("unknown type");
                    let handle = self.data.obj_pool.new_handle_with_assoc(uref);
                    self.data.vstack.push(Value::Ref(handle));
                }
                Code::ONewDict => {
                    let handle = self.data.obj_pool.new_dict_handle();
                    self.data.vstack.push(Value::Ref(handle));
                }
                Code::ONewArray => {
                    let handle = self.data.obj_pool.new_array_handle();
                    self.data.vstack.push(Value::Ref(handle));
                }
                Code::ODispose => {
                    let handle = usize::from(self.data.vstack.pop().expect("no object"));
                    self.data.obj_pool.dispose_handle(&handle);
                }
                Code::OCall(idx) => {
                    let name = &co.space.consts[*idx];
                    let argc = self.data.vstack.pop().expect("no argc");
                    let stack_size_after = self.data.vstack.len() - usize::from(argc);
                    let params = self.data.vstack.drain(stack_size_after..);
                    println!("calling {:?} with {:?}", name, params);
                    //let object = object_mut(&mut self.data);
                    //object.call();
                    //let cb = {
                    //    // TODO: ugh... remove this clone pls
                    //    object.lookup(aname).expect("no method found").clone()
                    //};
                    //// TODO: check if locals[0] is self => assign self = vstack.last()
                    //self.run_object(cb)?;
                }
                Code::OAppend => {
                    let value = self.data.vstack.pop().expect("no value");
                    object_mut(&mut self.data)
                        .as_indexable()
                        .unwrap()
                        .append(value);
                }
                Code::OGet(idx) => {
                    let aname = &co.space.consts[*idx];
                    let value = {
                        let mut object = object_mut(&mut self.data);
                        object
                            .as_indexable()
                            .unwrap()
                            .getk(&aname)
                            .expect("unknown attribute")
                            .clone()
                    };
                    self.data.vstack.push(value);
                }
                Code::OSet(idx) => {
                    let value = self.data.vstack.pop().expect("no value");
                    let mut object = object_mut(&mut self.data);
                    let aname = &co.space.consts[*idx];
                    object.as_indexable().unwrap().setk(&aname, value);
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

    pub fn run(&mut self, unit: &Unit) -> VmResult {
        // loads the programs main function
        let co = unit.code();

        self.data.units.load(unit)?;
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

fn object_mut(vm: &mut VmData) -> std::cell::RefMut<dyn ObjectProtocol> {
    match vm.vstack.last().expect("no object ref") {
        Value::Ref(handle) => vm.obj_pool.get_mut(&handle).unwrap().borrow_mut(),
        _ => unimplemented!(),
    }
}

fn frame(vm: &VmData) -> &VmFrame {
    vm.stack.last().expect("no last frame")
}

fn frame_mut(vm: &mut VmData) -> &mut VmFrame {
    vm.stack.last_mut().expect("no last frame")
}
