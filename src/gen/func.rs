use super::*;

// ---- example
// pseudocode:
//      f(x, y):
//          z = x + y
//          return z
// rust
//      gen::FunctionBuilder::new()
//          .with_args(vec!["x", "y"])      // TODO: is it `args` or `params` here? there was a difference...
//          .step(gen::Op::Add, "x", "y")
//          .store("z")
//          .end()
//          .build()
//
// ---- explanation

#[derive(PartialEq)]
enum Access {
    Read,
    Write,
}

pub type Function = CodeObject;

#[derive(Clone, Debug)]
pub struct FunctionBuilder {
    argc: usize,
    space: Space,
    seq: Sequence,
}

impl FunctionBuilder {
    pub fn new() -> Self {
        Self {
            argc: 0,
            space: Space::new(),
            seq: Sequence::new(),
        }
    }

    pub fn with_args<T>(mut self, args: Vec<T>) -> Self
    where
        T: std::string::ToString,
    {
        assert!(self.space.locals.is_empty());
        self.argc = args.len();
        // TODO: optimize this
        self.space.locals = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
        self
    }

    pub fn debug(mut self) -> Self {
        self.seq.push(Operation::new(OperationType::Debug));
        self
    }

    pub fn step(mut self, op: Operation) -> Self {
        for c in op.consts() {
            if !self.space.consts.contains(c) {
                self.space.consts.push(c.clone());
            }
        }

        if let Some(target) = op.target() {
            let name = target.as_name();
            if !self.space.locals.contains(&name) {
                self.space.locals.push(name.clone());
            }
        } else {
            // TODO: add error for empty operation
            unimplemented!();
        }

        self.seq.push(op);
        self
    }

    pub fn build(mut self) -> BuildResult<Function> {
        println!("building func {:#?}", self);

        let mut func = Function::new();
        func.argc = self.argc;
        func.space = self.space;

        for op in self.seq.iter() {
            let mut ops = op.ops();

            if let Some(inx) = op.as_inx() {
                let target = op.target().unwrap();
                let arg1 = ops.next().unwrap();
                if op.is_update() {
                    func.inner
                        .extend(translate_operand(&mut func.space, &target, Access::Read)?);
                }

                func.inner
                    .extend(translate_operand(&mut func.space, &arg1, Access::Read)?);

                func.inner.push(Code::Instruction(inx));

                func.inner
                    .extend(translate_operand(&mut func.space, &target, Access::Write)?);
            } else {
                match op.ty {
                    OperationType::Ass => {
                        let target = op.target().unwrap();
                        let arg1 = ops.next().unwrap();
                        func.inner
                            .extend(translate_operand(&mut func.space, &arg1, Access::Read)?);
                        func.inner.extend(translate_operand(
                            &mut func.space,
                            &target,
                            Access::Write,
                        )?);
                    }
                    OperationType::Debug => {
                        func.inner.extend(vec![
                            Code::Instruction(Instruction::Int),
                            Code::Value(Value::Ref(vm::Interrupt::Debug as usize)),
                        ]);
                    }
                    _ => unimplemented!(),
                }
            }
        }

        Ok(func)
    }
}

fn index_of<T>(ls: &Vec<T>, item: &T) -> usize
where
    T: PartialEq,
{
    ls.iter().position(|a| a == item).unwrap()
}

fn translate_operand(space: &mut Space, op: &Operand, acc: Access) -> BuildResult<CodeBlock> {
    match op {
        Operand::Name(n) if space.locals.contains(n) => {
            let idx = space.locals.iter().position(|local| local == n).unwrap();
            Ok(vec![
                Code::Instruction(if acc == Access::Write {
                    Instruction::Lpop
                } else {
                    Instruction::Lpush
                }),
                Code::Value(Value::Ref(idx)),
            ])
        }
        Operand::Name(n) => {
            let idx = if !space.globals.contains(n) {
                space.globals.push(n.clone());
                space.globals.len()
            } else {
                space.globals.iter().position(|global| global == n).unwrap()
            };
            Ok(vec![
                Code::Instruction(if acc == Access::Write {
                    Instruction::Gpop
                } else {
                    Instruction::Gpush
                }),
                Code::Value(Value::Ref(idx)),
            ])
        }
        Operand::Const(v) => {
            let idx = index_of(&space.consts, &v);
            Ok(vec![
                Code::Instruction(Instruction::Cpush),
                Code::Value(Value::Ref(idx)),
            ])
        }
    }
}
