use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Branch {
    cond: BlockDef,
    tblock: BlockDef,
    fblock: Option<BlockDef>,
}

impl Branch {
    pub fn new(cond: BlockDef, tblock: BlockDef, fblock: Option<BlockDef>) -> Self {
        Self {
            cond,
            tblock,
            fblock,
        }
    }
}

// the building blocks of lovm program definitions
#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    Branch(Box<Branch>),
    Embedded(Box<BlockDef>),
    Sequence(Sequence),
}

impl From<Sequence> for Block {
    fn from(from: Sequence) -> Self {
        Block::Sequence(from)
    }
}

impl From<Operation> for Block {
    fn from(from: Operation) -> Self {
        Block::Sequence(vec![from])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockDef {
    pub argc: usize,
    pub blocks: Vec<Block>,
    pub offsets: Offsets,
}

impl BlockDef {
    pub fn new() -> Self {
        Self {
            argc: 0,
            blocks: vec![],
            offsets: vec![],
        }
    }

    pub fn step<T: Into<Block>>(&mut self, smth: T) {
        // TODO: offsets will only be modified here. when build is called
        // we only have a read-only reference to these offsets
        self.blocks.push(smth.into())
    }

    pub fn branch(&mut self, cond: BlockDef, tblock: BlockDef, fblock: Option<BlockDef>) {
        //        self.step(Operation::push().op(cond).end());
        //
        //        let mut branch = CodeBuilder::new();
        //        branch.step(Operation::je().op(Operation::jf()).end());
        //        branch.embed(bcode);
        //        self.embed(branch);
        //
        //        if let Some(fbranch) = fbranch {
        //            let mut else_branch = CodeBuilder::new();
        //            else_branch.step(Operation::je());
        //            else_branch.embed(fbranch);
        //            self.embed(else_branch);
        //        }
        //
        let branch = Branch::new(cond, tblock, fblock);
        self.step(Block::Branch(Box::new(branch)));
    }

    fn compute_offsets(&self, codelen: usize) -> Vec<(usize, usize)> {
        self.offsets
            .iter()
            .filter_map(|(location, target)| match target {
                LinkTarget::Index(idx) => Some((*location, *idx)),
                LinkTarget::Location(loc) => match loc {
                    BranchLocation::Start => Some((*location, 0)),
                    BranchLocation::End => Some((*location, codelen)),
                    BranchLocation::Relative(_) => unimplemented!(),
                },
                _ => unimplemented!(),
            })
            .collect()
    }

    pub fn build(&self, _ensure_ret: bool) -> BuildResult<CodeObject> {
        // used for resolving branch offsets
        //let mut offsets = vec![];

        let mut func = CodeObject::new();
        //func.argc = self.argc.clone();
        //func.space = self.space.clone();

        // TODO: translate should produce a list of linktargets that
        // contain information about the codeobjects constants and
        // jump targets. the codeobject basically tells us, where
        // it expects a certain kind of value. we are then free to
        // exchange these code points freely.
        translate_sequence(&mut func, self.blocks.clone())?;

        let _final_offsets = self.compute_offsets(func.code.len());

        //func.link(final_offsets);

        //// generating phase is nearly over! compute the final offsets now.
        //let mut final_offsets: Vec<(usize, usize)> = vec![];

        //let codelen = func.code.len();

        //final_offsets.extend(
        //    offsets
        //    .into_iter()
        //    .filter_map(|(location, target)| match target {
        //        LinkTarget::Index(idx) => Some((location, idx)),
        //        LinkTarget::Location(loc) => match loc {
        //            BranchLocation::Start => Some((location, 0)),
        //            BranchLocation::End => Some((location, codelen)),
        //            BranchLocation::Relative(_) => unimplemented!(),
        //        },
        //        LinkTarget::Block(_) => None,
        //    }),
        //);

        //// resolve offsets for this function
        //for (location, link_arg) in final_offsets.into_iter() {
        //    match &mut func.code[location] {
        //        Code::Jmp(prev_idx)
        //            | Code::Jt(prev_idx)
        //            | Code::Jf(prev_idx)
        //            // only take uninitialized jumps for now
        //            if *prev_idx == std::usize::MAX =>
        //            {
        //                *prev_idx = link_arg;
        //            }
        //        e => if cfg!(debug_assertions) {
        //            println!("{} already resolved", e)
        //        }
        //    };
        //}

        //// TODO: check if last instruction already is return
        //if ensure_ret {
        //    match func.code.last() {
        //        Some(Code::Ret) | Some(Code::Jmp(_)) => {}
        //        _ => func.code.push(Code::Ret),
        //    }
        //}

        Ok(func)
    }
}

impl BlockDef {
    pub fn with_params<T>(&mut self, params: Vec<T>) -> &mut Self
    where
        T: std::string::ToString,
    {
        self.with_params_loose(params);
        // param order: last in, first out
        for i in (0..self.argc).rev() {
            //let param = self.space.locals[i].clone();
            //self.step(gen::Operation::ass().var(param.to_string()).end());
        }
        self
    }

    // does not enforce argument popping; needed when branches are compiled (?)
    pub fn with_params_loose<T>(&mut self, params: Vec<T>) -> &mut Self
    where
        T: std::string::ToString,
    {
        //assert!(self.space.locals.is_empty());
        self.argc = params.len();
        //self.space.locals = params.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
        self
    }
}

//    pub fn debug(&mut self) -> &mut Self {
//        self.seq.push(Operation::new(OperationType::Debug));
//        self
//    }
//
//    //fn jump(&mut self, target: LinkTarget, ty: OperationType) -> &mut Self {
//    //    let target = target.into();
//    //    match target {
//    //        LinkTarget::Index(idx) => {
//    //            self.seq.push(Operation::new(ty).op(idx).end());
//    //        }
//    //        LinkTarget::Block(bl) => {
//    //            self.seq
//    //                .push(Operation::new(ty).op(self.branches.len()).end());
//    //            self.branches.push(bl);
//    //        }
//    //        _ => unimplemented!(),
//    //    }
//    //    self
//    //}
//
//    //// method for `jmp` (jump) instruction
//    //pub fn branch<T>(&mut self, target: T) -> &mut Self
//    //where
//    //    T: Into<LinkTarget>,
//    //{
//    //    //self.jump(target.into(), OperationType::Jmp)
//    //}
//
//    pub fn branch(&mut self, cond: CodeBuilder, bcode: CodeBuilder, fbranch: Option<CodeBuilder>)
//    {
//        self.step(Operation::push().op(cond).end());
//
//        let mut branch = CodeBuilder::new();
//        //branch.step(Operation::je().op(Operation::jf()).end());
//        branch.embed(bcode);
//        println!("branch if {:?}", branch);
//        self.embed(branch);
//
//        if let Some(fbranch) = fbranch {
//            let mut else_branch = CodeBuilder::new();
//            else_branch.step(Operation::je());
//            else_branch.embed(fbranch);
//            println!("branch else {:?}", else_branch);
//            self.embed(else_branch);
//        }
//
//        println!("me now {:?}", self);
//    }
//
//    // method for `jt` (jump-if-true) instruction
//    pub fn branch_if(&mut self, tbranch: CodeBuilder) -> &mut Self
//    //where
//    //    T: Into<LinkTarget>,
//    {
//        //self.step(Operation::jf)
//        self.embed(tbranch);
//        self
//    }
//
//    //// method for `jf` (jump-if-false) instruction
//    //pub fn branch_else<T>(&mut self, target: T) -> &mut Self
//    //where
//    //    T: Into<LinkTarget>,
//    //{
//    //    //self.jump(target.into(), OperationType::Jf)
//    //}
//
//    pub fn step(&mut self, op: Operation) -> &mut Self {
//        for c in op.consts() {
//            if !self.space.consts.contains(c) {
//                self.space.consts.push(c.clone());
//            }
//        }
//
//        match op.ty {
//            OperationType::Ass => {
//                if let Some(target) = op.target() {
//                    let name = target.as_name();
//                    if !self.space.locals.contains(&name) {
//                        self.space.locals.push(name.clone());
//                    }
//                }
//            }
//            _ => {}
//        }
//
//        self.seq.push(op);
//        self
//    }
//
//    pub fn embed(&mut self, cb: CodeBuilder) -> &mut Self {
//        // assert argc == 0 (embed is not allowed for arguments)
//        assert_eq!(cb.argc, 0);
//        // merge spaces
//        self.space.merge(&cb.space);
//        // TODO: save self.len for later loop optimization
//        // TODO: loop from former self.len till end and adjust jumps
//        // append opertions from cb to self
//        self.seq.extend(cb.seq);
//        self
//    }
//
//    // TODO: the parameter obsfucates build calls; maybe remove it again
//    pub fn build(&self, ensure_ret: bool) -> BuildResult<CodeObject> {
//        // used for resolving branch offsets
//        let mut offsets = vec![];
//
//        let mut func = CodeObject::new();
//        func.argc = self.argc.clone();
//        func.space = self.space.clone();
//
//        translate_sequence(&mut func, self.seq.clone(), &mut offsets)?;
//
//        // generating phase is nearly over! compute the final offsets now.
//        let mut final_offsets: Vec<(usize, usize)> = vec![];
//
//        //// compile branches onto end of function
//        //for (bidx, branch) in self.branches.iter().enumerate() {
//        //    let bidx: LinkTarget = bidx.into();
//        //    let branch_co = branch.build(true)?;
//
//        //    // replace branch index with the branche's entry point inside `func` CodeObject
//        //    for (location, _) in offsets.iter().filter(|(_, i)| *i == bidx) {
//        //        final_offsets.push((*location, func.code.len().into()));
//        //    }
//
//        //    func.merge(&branch_co);
//        //}
//
//        let codelen = func.code.len();
//
//        final_offsets.extend(
//            offsets
//                .into_iter()
//                .filter_map(|(location, target)| match target {
//                    LinkTarget::Index(idx) => Some((location, idx)),
//                    LinkTarget::Location(loc) => match loc {
//                        BranchLocation::Start => Some((location, 0)),
//                        BranchLocation::End => Some((location, codelen)),
//                        BranchLocation::Relative(_) => unimplemented!(),
//                    },
//                    LinkTarget::Block(_) => None,
//                }),
//        );
//
//        // resolve offsets for this function
//        for (location, link_arg) in final_offsets.into_iter() {
//            match &mut func.code[location] {
//                Code::Jmp(prev_idx)
//                | Code::Jt(prev_idx)
//                | Code::Jf(prev_idx)
//                    // only take uninitialized jumps for now
//                    if *prev_idx == std::usize::MAX =>
//                {
//                    *prev_idx = link_arg;
//                }
//                e => if cfg!(debug_assertions) {
//                    println!("{} already resolved", e)
//                }
//            };
//        }
//
//        // TODO: check if last instruction already is return
//        if ensure_ret {
//            match func.code.last() {
//                Some(Code::Ret) | Some(Code::Jmp(_)) => {}
//                _ => func.code.push(Code::Ret),
//            }
//        }
//
//        Ok(func)
//    }
//}

impl From<Sequence> for BlockDef {
    fn from(seq: Sequence) -> Self {
        let mut new = Self::new();
        new.step(seq);
        new
    }
}

fn translate_sequence(
    func: &mut CodeObject,
    blocks: Vec<Block>,
    //offsets: &mut Offsets,
) -> BuildResult<()> {
    for block in blocks.iter() {
        match block {
            Block::Branch(_) => {}
            Block::Embedded(bl) => {
                let co = bl.build(false).unwrap();
                func.merge(&co);
            }
            Block::Sequence(seq) => {
                for op in seq.iter() {
                    translate_operation(func, op)?;
                }
            }
        }
    }
    Ok(())
}

fn translate(func: &mut CodeObject, op: &OpValue, acc: Access) -> BuildResult<()> {
    match op {
        OpValue::Operand(op) => translate_operand(func, op, acc),
        OpValue::Operation(op) => translate_operation(func, op),
        OpValue::BlockDef(block) => {
            func.merge(&block.build(false).unwrap());
            Ok(())
        }
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
            func.code.push(if acc == Access::Write {
                Code::LPop(idx)
            } else {
                Code::LPush(idx)
            });
        }
        Operand::Name(n) => {
            let idx = index_of(&mut func.space.globals, n);
            func.code.push(if acc == Access::Write {
                Code::GPop(idx)
            } else {
                Code::GPush(idx)
            });
        }
        Operand::Const(v) => {
            let idx = index_of(&mut func.space.consts, &v);
            func.code.push(Code::CPush(idx));
        }
    }
    Ok(())
}

fn translate_operation(func: &mut CodeObject, op: &Operation) -> BuildResult<()> {
    // TODO: as_inx should actually be a flatter branch
    if let Some(inx) = op.as_inx() {
        let mut ops = op.ops();
        if let Some(first) = ops.next() {
            translate(func, &first, Access::Read)?;
            if let Some(second) = ops.next() {
                translate(func, &second, Access::Read)?;
                func.code.push(inx);
                while let Some(next) = ops.next() {
                    translate(func, next, Access::Read)?;
                    func.code.push(inx);
                }
            } else {
                func.code.push(inx);
            }
        } else {
            func.code.push(inx);
        }
    } else {
        match &op.ty {
            OperationType::Ret => {
                for arg in op.ops() {
                    translate(func, arg, Access::Read)?;
                }
                func.code.push(Code::Ret);
            }
            OperationType::Ass => {
                if let Some(next) = op.rest().next() {
                    translate(func, &next, Access::Read)?;
                }
                translate_operand(func, &op.target().unwrap(), Access::Write)?;
            }
            OperationType::Call => {
                let fname = op.target().unwrap().as_name();
                for arg in op.rest() {
                    translate(func, arg, Access::Read)?;
                }
                // TODO: look at locals first
                let idx = index_of(&mut func.space.globals, &fname);
                func.code.push(Code::GCall(idx));
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
                func.code.push(op.as_inx().unwrap());
            }
            OperationType::Jmp | OperationType::Jt | OperationType::Jf => {
                let inx = match op.ty {
                    OperationType::Jmp => Code::Jmp(std::usize::MAX),
                    OperationType::Jt => Code::Jt(std::usize::MAX),
                    OperationType::Jf => Code::Jf(std::usize::MAX),
                    _ => unreachable!(),
                };
                if let Some(OpValue::Operand(_jmp_offset)) = op.ops().next() {
                    //offsets.push((
                    //    func.code.len(),
                    //    LinkTarget::Index(jmp_offset.as_const().clone().into()),
                    //));
                }
                func.code.push(inx);
            }
            OperationType::Js | OperationType::Je | OperationType::Jr => {
                let inx = match op.ops().next() {
                    Some(OpValue::Operation(Operation {
                        ty: OperationType::Jt,
                        ..
                    })) => Code::Jt(std::usize::MAX),
                    Some(OpValue::Operation(Operation {
                        ty: OperationType::Jf,
                        ..
                    })) => Code::Jf(std::usize::MAX),
                    _ => Code::Jmp(std::usize::MAX),
                };
                let _offset = match op.ty {
                    OperationType::Js => BranchLocation::Start,
                    OperationType::Je => BranchLocation::End,
                    OperationType::Jr => {
                        let rel = if let Some(OpValue::Operand(jmp_offset)) = op.rest().next() {
                            jmp_offset.as_const().clone().into()
                        } else {
                            panic!("not a valid relative value")
                        };
                        BranchLocation::Relative(rel)
                    }
                    _ => unreachable!(),
                };
                println!("pushing inx {:?} with {:?}", inx, op);
                //offsets.push((func.code.len(), offset.into()));
                func.code.push(inx);
            }
            OperationType::Int => match op.ops().next() {
                Some(OpValue::Operand(idx)) => {
                    let idx = idx.as_const().clone().into();
                    func.code.extend(vec![Code::Int(idx)])
                }
                _ => panic!("interrupt not specified"),
            },
            OperationType::Debug => {
                func.code
                    .extend(vec![Code::Int(vm::Interrupt::Debug as usize)]);
            }
            OperationType::ONew => {
                // first argument for onew is types name
                let ty_name = match op.ops().next() {
                    Some(OpValue::Operand(ty)) => ty.as_name(),
                    _ => unreachable!(),
                };
                let idx = index_of(&mut func.space.globals, &ty_name);
                func.code.extend(vec![Code::ONew(idx)]);
                // other arguments are initializers
                for arg in op.rest() {
                    // arg is either oset or oappend
                    translate(func, arg, Access::Read)?;
                }
            }
            OperationType::ONewArray => {
                func.code.extend(vec![Code::ONewArray]);
                for arg in op.ops() {
                    translate(func, arg, Access::Read)?;
                    // TODO: look at ONewDict. something is wrong here
                    func.code.push(Code::OAppend);
                }
            }
            OperationType::ONewDict => {
                func.code.extend(vec![Code::ONewDict]);
                for arg in op.ops() {
                    translate(func, arg, Access::Read)?;
                }
            }
            OperationType::OAppend => {
                for arg in op.ops() {
                    translate(func, arg, Access::Read)?;
                    func.code.push(Code::OAppend);
                }
            }
            OperationType::OGet => unimplemented!(),
            OperationType::OSet => {
                let mut it = op.ops();
                loop {
                    match (it.next(), it.next()) {
                        (Some(OpValue::Operand(Operand::Const(key))), Some(val)) => {
                            translate(func, val, Access::Read)?;
                            let idx = index_of(&mut func.space.consts, &key);
                            func.code.push(Code::OSet(idx));
                        }
                        (Some(key), _) => panic!("incorrect key `{:?}`", key),
                        _ => break,
                    }
                }
            }
            OperationType::OCall => {
                let mut argc = 0;

                // push arguments onto stack
                for arg in op.rest() {
                    translate(func, arg, Access::Read)?;
                    argc += 1;
                }
                // push argc onto stack
                let argc: OpValue = Operation::push().op(argc).end().into();
                translate(func, &argc, Access::Read)?;

                let fname = op.target().unwrap().as_name();
                let idx = index_of(&mut func.space.consts, &Value::from(fname.as_ref()));
                func.code.push(Code::OCall(idx));
            }
            OperationType::Embed => {
                if let Some(OpValue::BlockDef(child)) = op.ops().next() {
                    let child = child.build(false).unwrap();
                    func.merge(&child);
                } else {
                    panic!("not a valid embed argument")
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
        for (ln, step) in self.code.iter().enumerate() {
            writeln!(f, "\t{}\t{}", ln, step.to_string())?;
        }
        Ok(())
    }
}

impl std::fmt::Display for BlockDef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Code Builder(argc: {})", self.argc)?;
        //if !self.space.consts.is_empty() {
        //    writeln!(f, "\tconsts: {:?}", self.space.consts)?;
        //}
        //if !self.space.locals.is_empty() {
        //    writeln!(f, "\tlocals: {:?}", self.space.locals)?;
        //}
        //if !self.space.globals.is_empty() {
        //    writeln!(f, "\tglobals: {:?}", self.space.globals)?;
        //}
        ////if !self.branches.is_empty() {
        ////    writeln!(f, "\tbranches:")?;
        ////    for branch in self.branches.iter() {
        ////        writeln!(f, "\t\t{:?}", branch)?;
        ////    }
        ////}
        //writeln!(f, "\tcode:")?;
        //for step in self.seq.iter() {
        //    writeln!(f, "\t\t{}", step)?;
        //}
        Ok(())
    }
}
