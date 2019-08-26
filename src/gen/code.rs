use super::*;

impl CodeObject {
    pub fn merge(&mut self, other: &Self) {
        // at which location will the branch be added?
        let branch_offset = self.code.len();
        let mut other = other.clone();

        // when we merge two functions, we have to adjust the indices of `other`s constants, locals,
        // and globals using the following routine:
        // - for each instruction in `other`s body
        // - if instruction has an argument
        // - lookup the value behind index in `other`s body
        // - lookup the value inside `self`s body (will be added if not present)
        // - place the new index in argument location

        for inx in other.code.iter_mut() {
            // interrupts stay the same
            if let Code::Int(_) = inx {
                continue;
            }

            if let Some(prev_idx) = inx.arg() {
                let new_idx = match inx {
                    Code::CPush(_) => {
                        let prev_val = &other.space.consts[prev_idx];
                        index_of(&mut self.space.consts, prev_val)
                    }
                    Code::LPush(_) | Code::LPop(_) | Code::LCall(_) => {
                        let prev_val = &other.space.locals[prev_idx];
                        index_of(&mut self.space.locals, prev_val)
                    }
                    Code::GPush(_) | Code::GPop(_) | Code::GCall(_) => {
                        let prev_val = &other.space.globals[prev_idx];
                        // if ident was defined in parent frame, translate global operations
                        // to local scope
                        if self.space.locals.contains(prev_val) {
                            let new_idx = index_of(&mut self.space.locals, prev_val);
                            match inx.clone() {
                                Code::GPush(_) => *inx = Code::LPush(new_idx),
                                Code::GPop(_) => *inx = Code::LPop(new_idx),
                                Code::GCall(_) => *inx = Code::LCall(new_idx),
                                _ => unimplemented!(),
                            }
                            continue;
                        } else {
                            index_of(&mut self.space.globals, prev_val)
                        }
                    }
                    Code::Jmp(bidx) | Code::Jt(bidx) | Code::Jf(bidx) => {
                        // if this panics, no branch resolve was done
                        //assert!(*bidx < std::usize::MAX);

                        666
                        // jumps are now padded with the branch location
                        //*bidx + branch_offset
                    }
                    _ => panic!("`{:?}` not implemented for merge", inx),
                };

                *inx.arg_mut().unwrap() = new_idx;
            }
        }

        self.code.extend(other.code);
    }
}

////#[derive(Clone, Debug, PartialEq)]
////pub struct CodeBuilder {
////    argc: usize,
////    // TODO: branches must be abandoned, because the process of linking
////    // them at the end of a codebuilder is overcomplicated and inefficient.
////    //branches: Vec<CodeBuilder>,
////    space: Space,
////    seq: Sequence,
////}
//
//impl CodeBuilder {
//    pub fn new() -> Self {
//        Self {
//            argc: 0,
//            //branches: vec![],
//            space: Space::new(),
//            seq: Sequence::new(),
//        }
//    }
//
//    pub fn with_params<T>(&mut self, params: Vec<T>) -> &mut Self
//    where
//        T: std::string::ToString,
//    {
//        self.with_params_loose(params);
//        // param order: last in, first out
//        for i in (0..self.argc).rev() {
//            let param = self.space.locals[i].clone();
//            self.step(gen::Operation::ass().var(param.to_string()).end());
//        }
//        self
//    }
//
//    // does not enforce argument popping; needed when branches are compiled (?)
//    pub fn with_params_loose<T>(&mut self, params: Vec<T>) -> &mut Self
//    where
//        T: std::string::ToString,
//    {
//        assert!(self.space.locals.is_empty());
//        self.argc = params.len();
//        self.space.locals = params.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
//        self
//    }
//
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
