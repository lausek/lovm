use super::*;

use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Unit {
    pub(crate) codeblock: CodeBlock,
    pub(crate) labels: HashMap<Ident, LabelOffset>,
    pub(crate) src: String,
}

impl Unit {
    pub fn from(src: String) -> Self {
        Self {
            codeblock: CodeBlock::new(),
            labels: HashMap::new(),
            src,
        }
    }

    pub fn push_inx(&mut self, inx: Instruction) {
        self.codeblock.push(Code::Instruction(inx));
    }

    pub fn compile_operand(&mut self, op: Operand) -> Result<(), String> {
        // we have to push a placeholder value or the index will become corrupt
        let mut code = mkref(std::usize::MAX);

        match op {
            Operand::Ident(ident) => match self.labels.get_mut(&ident) {
                Some(LabelOffset::Resolved(off)) => code = mkref(*off),
                Some(LabelOffset::Unresolved(positions)) => {
                    positions.push((ident.clone(), self.codeblock.len()))
                }
                _ => {
                    self.labels.insert(
                        ident.clone(),
                        LabelOffset::Unresolved(vec![(ident.clone(), self.codeblock.len())]),
                    );
                }
            },
            Operand::Register(reg) => code = Code::Register(reg),
            Operand::Value(value) => code = Code::Value(value),
            Operand::Str(s) => {
                // TODO: write s as bytes in consequtive order to memory
                // TODO: insert reference to string pool here
                for c in s.bytes() {
                    self.codeblock.push(Code::Value(Value::I(c as i8)));
                }
                return Ok(());
            }
            Operand::Deref(_) => unreachable!(),
        }

        self.codeblock.push(code);
        Ok(())
    }

    pub fn compile_statement(&mut self, stmt: Statement) -> Result<(), Error> {
        match stmt.kw {
            Keyword::Dv => match stmt.args.get(0) {
                Some(Operand::Str(s)) => embed_string(s, &mut self.codeblock),
                Some(Operand::Value(value)) => {
                    let value = if let Some(ty) = stmt.ty {
                        value.cast(&Value::from_type(ty.into()))
                    } else {
                        value.clone()
                    };
                    self.codeblock.push(Code::Value(value));
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
                    self.codeblock
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
                // TODO: cast@ref could be used as shorthand for push <>; cast #5
                0 => self.codeblock.push(Code::Instruction(stmt.inx())),
                1 => {
                    self.codeblock.push(Code::Instruction(stmt.inx()));
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
                                self.codeblock
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

    pub fn declare_label(&mut self, label: Ident, off: usize) -> Result<(), Error> {
        match self
            .labels
            .insert(label.clone(), LabelOffset::Resolved(off))
        {
            Some(LabelOffset::Resolved(_)) => raise::redeclared(&label),
            // use reverse order to not invalidate indices
            Some(LabelOffset::Unresolved(positions)) => {
                for (_, pos) in positions.into_iter().rev() {
                    *self.codeblock.get_mut(pos).unwrap() = mkref(off);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn declare_value(&mut self, value: String) -> Result<(), String> {
        let value = Value::from_str(&value)?;
        self.codeblock.push(Code::Value(value));
        Ok(())
    }
}
