use super::*;

#[derive(PartialEq)]
enum Access {
    Read,
    Write,
}

pub type Function = Frame;

#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    pub argc: usize,
    pub space: Space,
    pub inner: CodeBlock,
    // used for resolving branch offsets
    offsets: Vec<(usize, usize)>,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            argc: 0,
            space: Space::new(),
            inner: CodeBlock::new(),
            offsets: vec![],
        }
    }

    pub fn merge(&mut self, other: &Self) {
        let mut other = other.clone();
        for inx in other.inner.iter_mut() {
            if let Some(prev_idx) = inx.arg() {
                let new_idx = match inx {
                    Instruction::Cpush(_) => {
                        let prev_val = &other.space.consts[prev_idx];
                        index_of(&mut self.space.consts, prev_val)
                    }
                    Instruction::Lpush(_) | Instruction::Lpop(_) => {
                        let prev_val = &other.space.locals[prev_idx];
                        index_of(&mut self.space.locals, prev_val)
                    }
                    Instruction::Gpush(_) | Instruction::Gpop(_) | Instruction::Gcall(_) => {
                        let prev_val = &other.space.globals[prev_idx];
                        // if ident was defined in parent frame, translate global operations
                        // to local scope
                        if self.space.locals.contains(prev_val) {
                            let new_idx = index_of(&mut self.space.locals, prev_val);
                            match inx.clone() {
                                Instruction::Gpush(_) => *inx = Instruction::Lpush(new_idx),
                                Instruction::Gpop(_) => *inx = Instruction::Lpop(new_idx),
                                Instruction::Gcall(_) => *inx = Instruction::Lcall(new_idx),
                                _ => unimplemented!(),
                            }
                            continue;
                        } else {
                            index_of(&mut self.space.globals, prev_val)
                        }
                    }
                    _ => unreachable!(),
                };
                if prev_idx != new_idx {
                    *inx.arg_mut().unwrap() = new_idx;
                }
            }
        }
        self.inner.extend(other.inner);
    }
}

impl From<Frame> for CodeObject {
    fn from(from: Frame) -> Self {
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
    branches: Vec<FunctionBuilder>,
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

    pub fn with_params<T>(mut self, params: Vec<T>) -> Self
    where
        T: std::string::ToString,
    {
        assert!(self.space.locals.is_empty());
        self.argc = params.len();
        // TODO: optimize this
        self.space.locals = params.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
        self
    }

    pub fn debug(&mut self) -> &mut Self {
        self.seq.push(Operation::new(OperationType::Debug));
        self
    }

    pub fn branch<T>(&mut self, jmp: Operation, func: T) -> &mut Self
    where
        T: Into<FunctionBuilder>,
    {
        self.seq.push(jmp);
        self.seq.push(Operation::jt().op(self.branches.len()).end());
        self.branches.push(func.into());
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

    pub fn build(&self) -> BuildResult<Frame> {
        let mut func = Frame::new();
        func.argc = self.argc.clone();
        func.space = self.space.clone();

        translate_sequence(&mut func, self.seq.clone())?;

        for (bidx, branch) in self.branches.iter().enumerate() {
            let boffset = func.inner.len();
            for (offset, _) in func.offsets.iter().filter(|(_, i)| *i == bidx) {
                *func.inner[*offset].arg_mut().unwrap() = boffset;
            }

            let mut branch_co = Frame::new();
            translate_sequence(&mut branch_co, branch.seq.clone())?;

            func.merge(&branch_co);
        }

        Ok(func)
    }
}

impl From<Sequence> for FunctionBuilder {
    fn from(seq: Sequence) -> Self {
        let mut new = Self::new();
        for c in seq.iter().flat_map(|op| op.consts()) {
            if !new.space.consts.contains(c) {
                new.space.consts.push(c.clone());
            }
        }
        new.seq = seq;
        new
    }
}

fn index_of<T>(ls: &mut Vec<T>, item: &T) -> usize
where
    T: Clone + PartialEq + std::fmt::Debug,
{
    match ls.iter().position(|a| a == item) {
        Some(idx) => idx,
        _ => {
            ls.push(item.clone());
            ls.len() - 1
        }
    }
}

fn translate_sequence(func: &mut Frame, seq: Sequence) -> BuildResult<()> {
    for op in seq.iter() {
        translate_operation(func, op)?;
    }
    Ok(())
}

fn translate(func: &mut Frame, op: &OpValue, acc: Access) -> BuildResult<()> {
    match op {
        OpValue::Operand(op) => translate_operand(func, op, acc),
        OpValue::Operation(op) => translate_operation(func, op),
    }
}

fn translate_operand(func: &mut Frame, op: &Operand, acc: Access) -> BuildResult<()> {
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
            let idx = index_of(&mut func.space.consts, &v);
            func.inner.push(Instruction::Cpush(idx));
        }
    }
    Ok(())
}

fn translate_operation(func: &mut Frame, op: &Operation) -> BuildResult<()> {
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
            OperationType::Ret => {
                for arg in op.ops() {
                    translate(func, arg, Access::Read);
                }
                func.inner.push(Instruction::Ret);
            }
            OperationType::Ass => {
                translate_operand(func, &op.target().unwrap(), Access::Write)?;
                translate(func, &op.rest().next().unwrap(), Access::Read)?;
            }
            OperationType::Call => {
                let fname = op.target().unwrap().as_name();
                for arg in op.rest() {
                    translate(func, arg, Access::Read)?;
                }
                let idx = index_of(&mut func.space.globals, &fname);
                func.inner.push(Instruction::Gcall(idx));
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
            OperationType::CmpEq
            | OperationType::CmpNe
            | OperationType::CmpGe
            | OperationType::CmpGt
            | OperationType::CmpLe
            | OperationType::CmpLt => {
                let target = op.target().unwrap();
                let arg1 = op.rest().next().unwrap();
                translate_operand(func, target, Access::Read)?;
                translate(func, arg1, Access::Read)?;
                func.inner.push(op.as_inx().unwrap());
            }
            OperationType::Jmp | OperationType::Jt | OperationType::Jf => {
                let inx = match op.ty {
                    OperationType::Jmp => Instruction::Jmp(std::usize::MAX),
                    OperationType::Jt => Instruction::Jt(std::usize::MAX),
                    OperationType::Jf => Instruction::Jf(std::usize::MAX),
                    _ => unreachable!(),
                };
                if let Some(target) = op.target() {
                    func.offsets
                        .push((func.inner.len(), target.as_const().clone().into()));
                }
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
