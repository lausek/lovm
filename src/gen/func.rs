use super::*;

impl CodeObject {
    pub fn merge(&mut self, other: &Self) {
        // at which location will the branch be added?
        let branch_offset = self.inner.len();
        let mut other = other.clone();

        // when we merge two functions, we have to adjust the indices of `other`s constants, locals,
        // and globals using the following routine:
        // - for each instruction in `other`s body
        // - if instruction has an argument
        // - lookup the value behind index in `other`s body
        // - lookup the value inside `self`s body (will be added if not present)
        // - place the new index in argument location

        for inx in other.inner.iter_mut() {
            if let Some(prev_idx) = inx.arg() {
                let new_idx = match inx {
                    Instruction::CPush(_) => {
                        let prev_val = &other.space.consts[prev_idx];
                        index_of(&mut self.space.consts, prev_val)
                    }
                    Instruction::LPush(_) | Instruction::LPop(_) | Instruction::LCall(_) => {
                        let prev_val = &other.space.locals[prev_idx];
                        index_of(&mut self.space.locals, prev_val)
                    }
                    Instruction::GPush(_) | Instruction::GPop(_) | Instruction::GCall(_) => {
                        let prev_val = &other.space.globals[prev_idx];
                        // if ident was defined in parent frame, translate global operations
                        // to local scope
                        if self.space.locals.contains(prev_val) {
                            let new_idx = index_of(&mut self.space.locals, prev_val);
                            match inx.clone() {
                                Instruction::GPush(_) => *inx = Instruction::LPush(new_idx),
                                Instruction::GPop(_) => *inx = Instruction::LPop(new_idx),
                                Instruction::GCall(_) => *inx = Instruction::LCall(new_idx),
                                _ => unimplemented!(),
                            }
                            continue;
                        } else {
                            index_of(&mut self.space.globals, prev_val)
                        }
                    }
                    Instruction::Jmp(bidx) | Instruction::Jt(bidx) | Instruction::Jf(bidx) => {
                        // if this panics, no branch resolve was done
                        assert!(*bidx < std::usize::MAX);

                        // jumps are now padded with the branch location
                        *bidx + branch_offset
                    }
                    _ => panic!("`{:?}` not implemented for merge", inx),
                };

                *inx.arg_mut().unwrap() = new_idx;
            }
        }

        self.inner.extend(other.inner);
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

    pub fn with_params<T>(self, params: Vec<T>) -> Self
    where
        T: std::string::ToString,
    {
        let mut new = self.with_params_loose(params);
        // param order: last in, first out
        for i in (0..new.argc).rev() {
            let param = new.space.locals[i].clone();
            new.step(gen::Operation::ass().var(param.to_string()).end());
        }
        new
    }

    // does not enforce argument popping; needed when branches are compiled (?)
    pub fn with_params_loose<T>(mut self, params: Vec<T>) -> Self
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

    fn jump(&mut self, target: BranchTarget, ty: OperationType) -> &mut Self {
        let target = target.into();
        match target {
            BranchTarget::Index(idx) => {
                self.seq.push(Operation::new(ty).op(idx).end());
            }
            BranchTarget::Block(bl) => {
                self.seq
                    .push(Operation::new(ty).op(self.branches.len()).end());
                self.branches.push(bl);
            }
        }
        self
    }

    // method for `jmp` (jump) instruction
    pub fn branch<T>(&mut self, target: T) -> &mut Self
    where
        T: Into<BranchTarget>,
    {
        self.jump(target.into(), OperationType::Jmp)
    }

    // method for `jt` (jump-if-true) instruction
    pub fn branch_if<T>(&mut self, target: T) -> &mut Self
    where
        T: Into<BranchTarget>,
    {
        self.jump(target.into(), OperationType::Jt)
    }

    // method for `jf` (jump-if-false) instruction
    pub fn branch_else<T>(&mut self, target: T) -> &mut Self
    where
        T: Into<BranchTarget>,
    {
        self.jump(target.into(), OperationType::Jf)
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

    pub fn build(&self) -> BuildResult<CodeObject> {
        // used for resolving branch offsets
        let mut offsets = vec![];

        let mut func = CodeObject::new();
        func.argc = self.argc.clone();
        func.space = self.space.clone();

        translate_sequence(&mut func, self.seq.clone(), &mut offsets)?;

        for (bidx, branch) in self.branches.iter().enumerate() {
            let branch_co = branch.build()?;

            // replace branch index with the branche's entry point inside `func` CodeObject
            for (_, link_arg) in offsets.iter_mut().filter(|(_, i)| *i == bidx) {
                *link_arg = func.inner.len();
            }

            func.merge(&branch_co);
        }

        for (offset, link_arg) in offsets.iter() {
            match &mut func.inner[*offset] {
                Instruction::Jmp(prev_idx)
                | Instruction::Jt(prev_idx)
                | Instruction::Jf(prev_idx)
                    // only take uninitialized jumps for now
                    if *prev_idx == std::usize::MAX =>
                {
                    *prev_idx = *link_arg;
                }
                e => println!("vad är {}", e),
            };
        }

        // TODO: check if last instruction already is return
        match func.inner.last() {
            Some(Instruction::Ret) | Some(Instruction::Jmp(_)) => {}
            _ => func.inner.push(Instruction::Ret),
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
            // TODO: add locals and globals (?)
        }
        new.seq = seq;
        new
    }
}

fn index_of<T>(ls: &mut Vec<T>, item: &T) -> usize
where
    T: Clone + Eq + std::fmt::Debug,
{
    match ls.iter().position(|a| a == item) {
        Some(idx) => idx,
        _ => {
            ls.push(item.clone());
            ls.len() - 1
        }
    }
}

fn translate_sequence(
    func: &mut CodeObject,
    seq: Sequence,
    offsets: &mut Vec<(usize, usize)>,
) -> BuildResult<()> {
    for op in seq.iter() {
        translate_operation(func, op, offsets)?;
    }
    Ok(())
}

fn translate(
    func: &mut CodeObject,
    op: &OpValue,
    acc: Access,
    offsets: &mut Vec<(usize, usize)>,
) -> BuildResult<()> {
    match op {
        OpValue::Operand(op) => translate_operand(func, op, acc),
        OpValue::Operation(op) => translate_operation(func, op, offsets),
    }
}

fn translate_operand(func: &mut CodeObject, op: &Operand, acc: Access) -> BuildResult<()> {
    match op {
        Operand::Name(n) if func.space.locals.contains(n) => {
            let idx = func
                .space
                .locals
                .iter()
                .position(|local| local == n)
                .unwrap();
            func.inner.push(if acc == Access::Write {
                Instruction::LPop(idx)
            } else {
                Instruction::LPush(idx)
            });
        }
        Operand::Name(n) => {
            let idx = index_of(&mut func.space.globals, n);
            func.inner.push(if acc == Access::Write {
                Instruction::GPop(idx)
            } else {
                Instruction::GPush(idx)
            });
        }
        Operand::Const(v) => {
            let idx = index_of(&mut func.space.consts, &v);
            func.inner.push(Instruction::CPush(idx));
        }
    }
    Ok(())
}

fn translate_operation(
    func: &mut CodeObject,
    op: &Operation,
    offsets: &mut Vec<(usize, usize)>,
) -> BuildResult<()> {
    // TODO: as_inx should actually be a flatter branch
    if let Some(inx) = op.as_inx() {
        let mut ops = op.ops();
        if let Some(first) = ops.next() {
            translate(func, &first, Access::Read, offsets)?;
            if let Some(second) = ops.next() {
                translate(func, &second, Access::Read, offsets)?;
                func.inner.push(inx);
                while let Some(next) = ops.next() {
                    translate(func, next, Access::Read, offsets)?;
                    func.inner.push(inx);
                }
            } else {
                func.inner.push(inx);
            }
        } else {
            func.inner.push(inx);
        }
    } else {
        match &op.ty {
            OperationType::Ret => {
                for arg in op.ops() {
                    translate(func, arg, Access::Read, offsets)?;
                }
                func.inner.push(Instruction::Ret);
            }
            OperationType::Ass => {
                if let Some(next) = op.rest().next() {
                    translate(func, &next, Access::Read, offsets)?;
                }
                translate_operand(func, &op.target().unwrap(), Access::Write)?;
            }
            OperationType::Call => {
                let fname = op.target().unwrap().as_name();
                for arg in op.rest() {
                    translate(func, arg, Access::Read, offsets)?;
                }
                // TODO: look at locals first
                let idx = index_of(&mut func.space.globals, &fname);
                func.inner.push(Instruction::GCall(idx));
            }
            OperationType::Push => {
                for arg in op.ops() {
                    translate(func, arg, Access::Read, offsets)?;
                }
            }
            OperationType::Pop => {
                for arg in op.ops() {
                    translate(func, arg, Access::Write, offsets)?;
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
                translate(func, arg1, Access::Read, offsets)?;
                func.inner.push(op.as_inx().unwrap());
            }
            OperationType::Jmp | OperationType::Jt | OperationType::Jf => {
                let inx = match op.ty {
                    OperationType::Jmp => Instruction::Jmp(std::usize::MAX),
                    OperationType::Jt => Instruction::Jt(std::usize::MAX),
                    OperationType::Jf => Instruction::Jf(std::usize::MAX),
                    _ => unreachable!(),
                };
                if let Some(OpValue::Operand(jmp_offset)) = op.ops().next() {
                    offsets.push((func.inner.len(), jmp_offset.as_const().clone().into()));
                }
                func.inner.push(inx);
            }
            OperationType::Debug => {
                func.inner
                    .extend(vec![Instruction::Int(vm::Interrupt::Debug as usize)]);
            }
            OperationType::ONew => {
                func.inner.extend(vec![Instruction::ONew]);
                for arg in op.ops() {
                    // arg is either oset or oappend
                    translate(func, arg, Access::Read, offsets)?;
                }
            }
            OperationType::ONewArray => {
                func.inner.extend(vec![Instruction::ONewArray]);
                for arg in op.ops() {
                    translate(func, arg, Access::Read, offsets)?;
                    // TODO: look at ONewDict. something is wrong here
                    func.inner.push(Instruction::OAppend);
                }
            }
            OperationType::ONewDict => {
                func.inner.extend(vec![Instruction::ONewDict]);
                for arg in op.ops() {
                    translate(func, arg, Access::Read, offsets)?;
                }
            }
            OperationType::OAppend => {
                for arg in op.ops() {
                    translate(func, arg, Access::Read, offsets)?;
                    func.inner.push(Instruction::OAppend);
                }
            }
            OperationType::OGet => unimplemented!(),
            OperationType::OSet => {
                let mut it = op.ops();
                loop {
                    match (it.next(), it.next()) {
                        (Some(OpValue::Operand(Operand::Const(key))), Some(val)) => {
                            translate(func, val, Access::Read, offsets)?;
                            let idx = index_of(&mut func.space.consts, &key);
                            func.inner.push(Instruction::OSet(idx));
                        }
                        (Some(key), _) => panic!("incorrect key `{:?}`", key),
                        _ => break,
                    }
                }
            }
            other => panic!("`{:?}` not yet implemented", other),
        }
    }
    Ok(())
}

impl std::fmt::Display for CodeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "CodeObject(argc: {})", self.argc)?;
        if !self.space.consts.is_empty() {
            writeln!(f, "\tconsts: {:?}", self.space.consts)?;
        }
        if !self.space.locals.is_empty() {
            writeln!(f, "\tlocals: {:?}", self.space.locals)?;
        }
        if !self.space.globals.is_empty() {
            writeln!(f, "\tglobals: {:?}", self.space.globals)?;
        }
        writeln!(f, "\tcode:")?;
        for (ln, step) in self.inner.iter().enumerate() {
            writeln!(f, "\t{}\t{}", ln, step.to_string())?;
        }
        Ok(())
    }
}

impl std::fmt::Display for FunctionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Function Builder(argc: {})", self.argc)?;
        if !self.space.consts.is_empty() {
            writeln!(f, "\tconsts: {:?}", self.space.consts)?;
        }
        if !self.space.locals.is_empty() {
            writeln!(f, "\tlocals: {:?}", self.space.locals)?;
        }
        if !self.space.globals.is_empty() {
            writeln!(f, "\tglobals: {:?}", self.space.globals)?;
        }
        if !self.branches.is_empty() {
            writeln!(f, "\tbranches:")?;
            for branch in self.branches.iter() {
                writeln!(f, "\t\t{:?}", branch)?;
            }
        }
        writeln!(f, "\tcode:")?;
        for step in self.seq.iter() {
            writeln!(f, "\t\t{}", step)?;
        }
        Ok(())
    }
}
