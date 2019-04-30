use super::*;

#[derive(PartialEq)]
enum Access {
    Read,
    Write,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub argc: usize,
    pub space: Space,
    pub inner: CodeBlock,
    // used for resolving branch offsets
    offsets: Vec<(usize, usize)>,
}

impl Function {
    pub fn new() -> Self {
        Self {
            argc: 0,
            space: Space::new(),
            inner: CodeBlock::new(),
            offsets: vec![],
        }
    }
}

impl From<Function> for CodeObject {
    fn from(from: Function) -> Self {
        Self {
            argc: from.argc,
            space: from.space,
            inner: from.inner,
        }
    }
}

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

        match op.ty {
            OperationType::Ass => {
                if let Some(target) = op.target() {
                    let name = target.as_name();
                    if !self.space.locals.contains(&name) {
                        self.space.locals.push(name.clone());
                    }
                }
            }
            _ => {}
        }

        self.seq.push(op);
        self
    }

    pub fn build(&self) -> BuildResult<Function> {
        let mut func = Function::new();
        func.argc = self.argc.clone();
        func.space = self.space.clone();
        translate_sequence(&mut func, self.seq.clone())?;
        println!("building func {:#?}", func);

        for (bidx, branch) in self.branches.iter().enumerate() {
            let boffset = func.inner.len();
            for (offset, _) in func.offsets.iter().filter(|(_, i)| *i == bidx) {
                func.inner[*offset].set_arg(boffset);
            }

            translate_sequence(&mut func, branch.clone())?;
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

fn translate_sequence(func: &mut Function, seq: Sequence) -> BuildResult<()> {
    for op in seq.iter() {
        translate_operation(func, op)?;
    }
    Ok(())
}

fn translate(func: &mut Function, op: &OpValue, acc: Access) -> BuildResult<()> {
    match op {
        OpValue::Operand(op) => translate_operand(func, op, acc),
        OpValue::Operation(op) => translate_operation(func, op),
    }
}

fn translate_operand(func: &mut Function, op: &Operand, acc: Access) -> BuildResult<()> {
    match op {
        Operand::Name(n) if func.space.locals.contains(n) => {
            let idx = func
                .space
                .locals
                .iter()
                .position(|local| local == n)
                .unwrap();
            func.inner.push(if acc == Access::Write {
                Instruction::Lpop(idx)
            } else {
                Instruction::Lpush(idx)
            });
        }
        Operand::Name(n) => {
            let idx = if !func.space.globals.contains(n) {
                let idx = func.space.globals.len();
                func.space.globals.push(n.clone());
                idx
            } else {
                func.space
                    .globals
                    .iter()
                    .position(|global| global == n)
                    .unwrap()
            };
            func.inner.push(if acc == Access::Write {
                Instruction::Gpop(idx)
            } else {
                Instruction::Gpush(idx)
            });
        }
        Operand::Const(v) => {
            let idx = index_of(&func.space.consts, &v);
            func.inner.push(Instruction::Cpush(idx));
        }
    }
    Ok(())
}

fn translate_operation(func: &mut Function, op: &Operation) -> BuildResult<()> {
    if let Some(inx) = op.as_inx() {
        let mut ops = op.ops();
        if let Some(first) = ops.next() {
            translate(func, &first, Access::Read)?;
            if let Some(second) = ops.next() {
                translate(func, &second, Access::Read)?;
                func.inner.push(inx);
                while let Some(next) = ops.next() {
                    translate(func, next, Access::Read)?;
                    func.inner.push(inx);
                }
            } else {
                func.inner.push(inx);
            }
        } else {
            func.inner.push(inx);
        }
    } else {
        match op.ty {
            OperationType::Ret => func.inner.push(Instruction::Ret),
            OperationType::Ass => {
                translate_operand(func, &op.target().unwrap(), Access::Write)?;
                translate(func, &op.rest().next().unwrap(), Access::Read)?;
            }
            OperationType::Call => {
                for arg in op.rest() {
                    translate(func, arg, Access::Read)?;
                }
                translate_operand(func, &op.target().unwrap(), Access::Read)?;
                func.inner.push(Instruction::Call);
            }
            OperationType::Push => {
                for arg in op.ops() {
                    translate(func, arg, Access::Read)?;
                }
            }
            OperationType::Pop => {
                for arg in op.ops() {
                    translate(func, arg, Access::Write)?;
                }
            }
            OperationType::Cmp => {
                let target = op.target().unwrap();
                let arg1 = op.rest().next().unwrap();
                translate_operand(func, target, Access::Read)?;
                translate(func, arg1, Access::Read)?;
                func.inner.push(Instruction::Cmp);
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
                func.offsets
                    .push((func.inner.len(), target.as_const().clone().into()));
                func.inner.push(inx);
            }
            OperationType::Debug => {
                func.inner
                    .extend(vec![Instruction::Int(vm::Interrupt::Debug as usize)]);
            }
            _ => unimplemented!(),
        }
    }
    Ok(())
}
