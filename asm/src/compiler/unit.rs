use super::*;

#[derive(Clone, Debug)]
pub struct Unit {
    pub(crate) code: CodeBlock,
    pub(crate) labels: HashMap<Ident, Label>,
    pub(crate) path: Option<String>,
    pub(crate) src: String,
    pub(crate) sub_units: Vec<Unit>,
}

impl Unit {
    pub fn from_path(src: String, path: String) -> Self {
        Self {
            code: CodeBlock::new(),
            labels: HashMap::new(),
            path: Some(path),
            src,
            sub_units: vec![],
        }
    }

    pub fn from(src: String) -> Self {
        Self {
            code: CodeBlock::new(),
            labels: HashMap::new(),
            path: None,
            src,
            sub_units: vec![],
        }
    }

    pub fn push_inx(&mut self, inx: Instruction) {
        self.code.push(Code::Instruction(inx));
    }

    pub fn compile_operand(&mut self, op: Operand) -> Result<(), String> {
        // we have to push a placeholder value or the index will become corrupt
        let mut code = mkref(std::usize::MAX);

        // TODO: put operand into constant pool if it doesn't exist yet

        match op {
            Operand::Ident(ident) => match self.labels.get_mut(&ident) {
                Some(label) => label.locations.push((ident.clone(), self.code.len())),
                _ => {
                    let label = Label::new().location(&ident, self.code.len());
                    self.labels.insert(ident.clone(), label);
                }
            },
            Operand::Register(reg) => code = Code::Register(reg),
            Operand::Value(value) => code = Code::Value(value),
            Operand::Str(s) => {
                for c in s.bytes() {
                    self.code.push(Code::Value(Value::I(c as i8)));
                }
                return Ok(());
            }
            Operand::Deref(_) => unreachable!(),
        }

        self.code.push(code);
        Ok(())
    }

    pub fn compile_statement(&mut self, stmt: Statement) -> Result<(), Error> {
        match stmt.kw {
            Keyword::Dv => match stmt.args.get(0) {
                Some(Operand::Str(s)) => embed_string(s, &mut self.code),
                Some(Operand::Value(value)) => {
                    let value = if let Some(ty) = stmt.ty {
                        value.cast(&Value::from_type(ty.into()))
                    } else {
                        value.clone()
                    };
                    self.code.push(Code::Value(value));
                }
                Some(arg) => {
                    return raise::not_a_value(arg.clone());
                }
                None => return raise::expected_either_got(&["label", "const"], None),
            },
            Keyword::Cmp => {
                self.push_inx(Instruction::Push);
                self.compile_operand(stmt.args[0].clone())?;
                self.push_inx(Instruction::Push);
                self.compile_operand(stmt.args[1].clone())?;
                self.push_inx(Instruction::Cmp);
            }
            Keyword::Cast => {
                self.push_inx(Instruction::Push);
                self.compile_operand(stmt.args[0].clone())?;
                self.push_inx(Instruction::Cast);
                self.compile_operand(stmt.args[1].clone())?;
                self.push_inx(Instruction::Pop);
                self.compile_operand(stmt.args[0].clone())?;
            }
            Keyword::Mov => {
                /*
                mov a, *b should become:
                    push b
                    load
                    pop a

                mov *a, b should become:
                    push b
                    push a
                    store

                mov *a, *b should become:
                    push b
                    load
                    push a
                    store
                */
                let x1 = stmt.args[0].clone();
                let x2 = stmt.args[1].clone();
                if let Operand::Deref(x2) = x2 {
                    self.push_inx(Instruction::Push);
                    self.compile_operand(*x2)?;
                    self.push_inx(Instruction::Load);
                } else {
                    self.push_inx(Instruction::Push);
                    self.compile_operand(x2)?;
                }

                if let Some(ty) = stmt.ty {
                    self.push_inx(Instruction::Cast);
                    self.code
                        .push(Code::Value(Value::I(ty.clone().into())));
                }

                if let Operand::Deref(x1) = x1 {
                    self.push_inx(Instruction::Push);
                    self.compile_operand(*x1)?;
                    self.push_inx(Instruction::Store);
                } else {
                    self.push_inx(Instruction::Pop);
                    self.compile_operand(x1)?;
                }
            }
            _ => match stmt.argc() {
                // TODO: add `ret@i <stack_last>` for typed return (?)
                // TODO: cast@ref could be used as shorthand for `push <stack_last>; cast #5`
                0 => self.code.push(Code::Instruction(stmt.inx())),
                1 => {
                    self.code.push(Code::Instruction(stmt.inx()));
                    self.compile_operand(stmt.args[0].clone())?;
                }
                _n => {
                    let inx = stmt.inx();
                    let kw = stmt.kw;
                    let x1 = stmt.args[0].clone();
                    let x2 = stmt.args[1].clone();
                    match kw {
                        Keyword::Add
                        | Keyword::Sub
                        | Keyword::Mul
                        | Keyword::Div
                        | Keyword::Rem
                        | Keyword::Pow
                        | Keyword::Neg
                        | Keyword::And
                        | Keyword::Or
                        | Keyword::Xor
                        | Keyword::Shl
                        | Keyword::Shr => {
                            // push arg1
                            self.push_inx(Instruction::Push);
                            self.compile_operand(x1.clone())?;

                            if let Some(ty) = stmt.ty {
                                self.push_inx(Instruction::Cast);
                                self.code
                                    .push(Code::Value(Value::I(ty.clone().into())));
                            }

                            // push arg2
                            self.push_inx(Instruction::Push);
                            self.compile_operand(x2)?;

                            // opcode
                            self.push_inx(inx);

                            // restore value to target register
                            self.push_inx(Instruction::Pop);
                            self.compile_operand(x1)?;
                        }
                        _ => unreachable!(),
                    }
                }
            },
        }

        Ok(())
    }

    pub fn declare_label(&mut self, ident: Ident, off: usize) -> Result<(), Error> {
        match self.labels.get_mut(&ident) {
            Some(label) if label.decl.is_none() => label.decl = Some((ident.clone(), off)),
            Some(_) => return raise::redeclared(&ident),
            _ => {
                let label = Label::new().declaration(&ident, off);
                self.labels.insert(ident, label);
            }
        }
        Ok(())
    }

    pub fn link(&mut self) -> Result<(), Error> {
        let mut errs = vec![];
        //println!("linking {:?}", self.path);
        //println!("sub_units {:#?}", self.sub_units);

        let mut link_offset = self.code.len();

        for sub_unit in self.sub_units.iter_mut() {
            sub_unit.labels.iter_mut().for_each(|(_, label)| {
                label.decl.as_mut().unwrap().1 += link_offset;
            });
            sub_unit.link()?;
            self.code.extend(sub_unit.code.clone());
            merge_labels(&mut self.labels, &sub_unit.labels)?;
            link_offset = self.code.len();
        }

        for (_, label) in self.labels.iter() {
            if let Some((_, off)) = label.decl {
                for (_, idx) in label.locations.iter().rev() {
                    *self.code.get_mut(*idx).unwrap() = Code::Value(Value::Ref(off));
                }
            } else {
                for (ident, _) in label.locations.iter() {
                    errs.push(raise::not_declared::<CompileResult>(ident).err().unwrap());
                }
            }
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs.into())
        }
    }
}

fn merge_labels(
    ctx: &mut HashMap<Ident, Label>,
    scope: &HashMap<Ident, Label>,
) -> Result<(), Error> {
    for (key, value) in scope.iter() {
        match ctx.get_mut(&key) {
            // FIXME: raise `redeclared`
            Some(label) if label.decl.is_none() && value.decl.is_some() => {
                label.decl = value.decl.clone();
            }
            Some(_) => panic!("label {:?} redeclared", key),
            _ => {}
        }
    }
    Ok(())
}
