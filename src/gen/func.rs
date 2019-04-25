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
    branches: Vec<Sequence>,
    space: Space,
    seq: Sequence,
}

impl FunctionBuilder {
    pub fn new() -> Self {
        Self {
            argc: 0,
            branches: vec![],
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

    pub fn branch(mut self, jmp: Operation, seq: Sequence) -> Self {
        for c in seq.iter().flat_map(|op| op.consts()) {
            if !self.space.consts.contains(c) {
                self.space.consts.push(c.clone());
            }
        }

        self.seq.push(jmp.op(self.branches.len()));
        self.branches.push(seq);
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
        }

        self.seq.push(op);
        self
    }

    pub fn build(mut self) -> BuildResult<Function> {
        println!("building func {:#?}", self);

        // used for resolving branch offsets
        // arg1: branch index, arg2: func.inner index
        let mut offsets: Vec<(usize, usize)> = vec![];

        let mut func = Function::new();
        func.argc = self.argc;
        func.space = self.space;
        func.inner = translate_sequence(&mut func.space, self.seq, &mut offsets)?;

        for (bidx, branch) in self.branches.into_iter().enumerate() {
            let boffset = func.inner.len();
            for (offset, _) in offsets.iter().filter(|(_, i)| *i == bidx) {
                func.inner[*offset] = mkref(boffset);
            }

            let branch_co = translate_sequence(&mut func.space, branch, &mut offsets)?;
            func.inner.extend(branch_co);
        }

        Ok(func)
    }
}

fn index_of<T>(ls: &Vec<T>, item: &T) -> usize
where
    T: PartialEq + std::fmt::Debug,
{
    ls.iter().position(|a| a == item).unwrap()
}

fn translate_sequence(
    space: &mut Space,
    seq: Sequence,
    offsets: &mut Vec<(usize, usize)>,
) -> BuildResult<CodeBlock> {
    let mut co = vec![];
    for op in seq.iter() {
        if let Some(inx) = op.as_inx() {
            if let Some(target) = op.target() {
                let arg1 = op.rest().next().unwrap();

                co.extend(translate_operand(space, &target, Access::Read)?);
                co.extend(translate_operand(space, &arg1, Access::Read)?);
                co.push(Code::Instruction(inx));

                if op.is_update() {
                    co.extend(translate_operand(space, &target, Access::Write)?);
                }
            } else {
                co.push(Code::Instruction(inx));
            }
        } else {
            match op.ty {
                OperationType::Ass => {
                    let target = op.target().unwrap();
                    let arg1 = op.rest().next().unwrap();
                    co.extend(translate_operand(space, &arg1, Access::Read)?);
                    co.extend(translate_operand(space, &target, Access::Write)?);
                }
                OperationType::Call => {
                    for arg in op.rest() {
                        co.extend(translate_operand(space, &arg, Access::Read)?);
                    }
                    co.extend(translate_operand(
                        space,
                        &op.target().unwrap(),
                        Access::Read,
                    )?);
                    co.push(Code::Instruction(Instruction::Call));
                }
                OperationType::Ret => {
                    co.push(Code::Instruction(Instruction::Ret));
                }
                OperationType::Cmp => {
                    let target = op.target().unwrap();
                    let arg1 = op.rest().next().unwrap();
                    co.extend(translate_operand(space, &arg1, Access::Read)?);
                    co.extend(translate_operand(space, &target, Access::Read)?);
                    co.push(Code::Instruction(Instruction::Cmp));
                }
                OperationType::Jmp
                | OperationType::Jeq
                | OperationType::Jne
                | OperationType::Jge
                | OperationType::Jgt
                | OperationType::Jle
                | OperationType::Jlt => {
                    let target = op.target().unwrap();
                    let inx = match op.ty {
                        OperationType::Jmp => Instruction::Jmp,
                        OperationType::Jeq => Instruction::Jeq,
                        OperationType::Jne => Instruction::Jne,
                        OperationType::Jge => Instruction::Jge,
                        OperationType::Jgt => Instruction::Jgt,
                        OperationType::Jle => Instruction::Jle,
                        OperationType::Jlt => Instruction::Jlt,
                        _ => unreachable!(),
                    };
                    co.push(Code::Instruction(inx));

                    offsets.push((co.len(), target.as_const().clone().into()));
                    co.push(mkref(std::usize::MAX));
                }
                OperationType::Debug => {
                    co.extend(vec![
                        Code::Instruction(Instruction::Int),
                        Code::Value(Value::Ref(vm::Interrupt::Debug as usize)),
                    ]);
                }
                _ => unimplemented!(),
            }
        }
    }
    Ok(co)
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
