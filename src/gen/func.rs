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

    pub fn debug(&mut self) -> &mut Self {
        self.seq.push(Operation::new(OperationType::Debug));
        self
    }

    pub fn branch(&mut self, mut jmp: Operation, seq: Sequence) -> &mut Self {
        for c in seq.iter().flat_map(|op| op.consts()) {
            if !self.space.consts.contains(c) {
                self.space.consts.push(c.clone());
            }
        }

        self.seq.push(jmp.op(self.branches.len()).end());
        self.branches.push(seq);
        self
    }

    pub fn step(&mut self, op: Operation) -> &mut Self {
        for c in op.consts() {
            if !self.space.consts.contains(c) {
                self.space.consts.push(c.clone());
            }
        }

        if op.is_update() {
            if let Some(target) = op.target() {
                let name = target.as_name();
                if !self.space.locals.contains(&name) {
                    self.space.locals.push(name.clone());
                }
            }
        }

        self.seq.push(op);
        self
    }

    pub fn build(&self) -> BuildResult<Function> {
        println!("building func {:#?}", self);

        // used for resolving branch offsets
        // arg1: branch index, arg2: func.inner index
        let mut offsets: Vec<(usize, usize)> = vec![];

        let mut func = Function::new();
        func.argc = self.argc.clone();
        func.space = self.space.clone();
        func.inner = translate_sequence(&mut func.space, self.seq.clone(), &mut offsets)?;

        for (bidx, branch) in self.branches.iter().enumerate() {
            let boffset = func.inner.len();
            for (offset, _) in offsets.iter().filter(|(_, i)| *i == bidx) {
                func.inner[*offset].set_arg(boffset);
            }

            let branch_co = translate_sequence(&mut func.space, branch.clone(), &mut offsets)?;
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
            for arg in op.ops() {
                co.extend(translate_operand(space, &arg, Access::Read)?);
                co.push(inx);
            }

            if op.is_update() {
                co.extend(translate_operand(
                    space,
                    op.target().as_ref().unwrap(),
                    Access::Write,
                )?);
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
                    co.push(Instruction::Call);
                }
                OperationType::Ret => {
                    co.push(Instruction::Ret);
                }
                OperationType::Cmp => {
                    let target = op.target().unwrap();
                    let arg1 = op.rest().next().unwrap();
                    co.extend(translate_operand(space, &arg1, Access::Read)?);
                    co.extend(translate_operand(space, &target, Access::Read)?);
                    co.push(Instruction::Cmp);
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
                        OperationType::Jmp => Instruction::Jmp(std::usize::MAX),
                        OperationType::Jeq => Instruction::Jeq(std::usize::MAX),
                        OperationType::Jne => Instruction::Jne(std::usize::MAX),
                        OperationType::Jge => Instruction::Jge(std::usize::MAX),
                        OperationType::Jgt => Instruction::Jgt(std::usize::MAX),
                        OperationType::Jle => Instruction::Jle(std::usize::MAX),
                        OperationType::Jlt => Instruction::Jlt(std::usize::MAX),
                        _ => unreachable!(),
                    };
                    offsets.push((co.len(), target.as_const().clone().into()));
                    co.push(inx);
                }
                OperationType::Debug => {
                    co.extend(vec![Instruction::Int(vm::Interrupt::Debug as usize)]);
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
            Ok(vec![if acc == Access::Write {
                Instruction::Lpop(idx)
            } else {
                Instruction::Lpush(idx)
            }])
        }
        Operand::Name(n) => {
            let idx = if !space.globals.contains(n) {
                space.globals.push(n.clone());
                space.globals.len()
            } else {
                space.globals.iter().position(|global| global == n).unwrap()
            };
            Ok(vec![if acc == Access::Write {
                Instruction::Gpop(idx)
            } else {
                Instruction::Gpush(idx)
            }])
        }
        Operand::Const(v) => {
            let idx = index_of(&space.consts, &v);
            Ok(vec![Instruction::Cpush(idx)])
        }
    }
}
