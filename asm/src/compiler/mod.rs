pub use super::*;

pub mod error;
mod objects;
mod parser;
mod unit;

pub use self::error::*;
pub use self::objects::*;
pub use self::parser::*;
pub use self::unit::*;

use lovm::value::Value;

use std::collections::HashMap;

pub type CompileResult = Result<Unit, Error>;

const fn mkref(raw: usize) -> Code {
    Code::Value(Value::Ref(raw))
}

// if a label lookup doesn't deliver a result while generating, remember the labels
// name and the current generation offset for later. after all generation is done, we will
// go for a final lookup and insert the now existing result at the index on the codeblock.
#[derive(Clone, Debug)]
pub enum LabelOffset {
    // the label already occurred while compiling the program; this contains
    // its offset inside the codeblock
    Resolved(usize),
    // the label is still unknown. contains a list of indices where we have
    // to insert the resolved index
    Unresolved(Vec<(Ident, usize)>),
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&mut self, src: &str) -> CompileResult {
        let mut unit = Unit::from(src.to_string());
        let ast = parser::parse(&unit.src)?;

        for step in ast.into_iter() {
            match step {
                Ast::Label(ident) => unit.declare_label(ident, unit.codeblock.len())?,
                Ast::Declare(value) => unit.declare_value(value)?,
                Ast::Statement(stmt) => self.compile_statement(stmt, &mut unit)?,
            }
        }

        self.check_resolved(&unit)?;

        Ok(unit)
    }

    fn compile_statement(&self, stmt: Statement, unit: &mut Unit) -> Result<(), Error> {
        match stmt.kw {
            Keyword::Dv => match stmt.args.get(0) {
                Some(Operand::Value(value)) => {
                    // TODO: allow static type casting here
                    unit.codeblock.push(Code::Value(*value));
                }
                Some(arg) => {
                    return raise::not_a_value(arg.clone());
                }
                None => return raise::expected_either_got(&["label", "const"], None),
            },
            Keyword::Cmp => {
                unit.push_inx(Instruction::Push);
                unit.compile_operand(stmt.args[0].clone())?;
                unit.push_inx(Instruction::Push);
                unit.compile_operand(stmt.args[1].clone())?;
                unit.push_inx(Instruction::Cmp);
            }
            Keyword::Coal => {
                unit.push_inx(Instruction::Push);
                unit.compile_operand(stmt.args[0].clone())?;
                unit.push_inx(Instruction::Coal);
                unit.compile_operand(stmt.args[1].clone())?;
                unit.push_inx(Instruction::Pop);
                unit.compile_operand(stmt.args[0].clone())?;
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
                    unit.push_inx(Instruction::Push);
                    unit.compile_operand(*x2)?;
                    unit.push_inx(Instruction::Load);
                } else {
                    unit.push_inx(Instruction::Push);
                    unit.compile_operand(x2)?;
                }

                if let Some(ty) = stmt.ty {
                    unit.push_inx(Instruction::Coal);
                    unit.codeblock
                        .push(Code::Value(Value::I(ty.clone().into())));
                }

                if let Operand::Deref(x1) = x1 {
                    unit.push_inx(Instruction::Push);
                    unit.compile_operand(*x1)?;
                    unit.push_inx(Instruction::Store);
                } else {
                    unit.push_inx(Instruction::Pop);
                    unit.compile_operand(x1)?;
                }
            }
            _ => match stmt.argc() {
                // TODO: coal@ref could be used as shorthand for push <>; coal #5
                0 => unit.codeblock.push(Code::Instruction(stmt.inx())),
                1 => {
                    unit.codeblock.push(Code::Instruction(stmt.inx()));
                    unit.compile_operand(stmt.args[0].clone())?;
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
                            unit.push_inx(Instruction::Push);
                            unit.compile_operand(x1.clone())?;

                            if let Some(ty) = stmt.ty {
                                unit.push_inx(Instruction::Coal);
                                unit.codeblock
                                    .push(Code::Value(Value::I(ty.clone().into())));
                            }

                            // push arg2
                            unit.push_inx(Instruction::Push);
                            unit.compile_operand(x2)?;

                            // opcode
                            unit.push_inx(inx);

                            // restore value to target register
                            unit.push_inx(Instruction::Pop);
                            unit.compile_operand(x1)?;
                        }
                        _ => unreachable!(),
                    }
                }
            },
        }

        Ok(())
    }

    fn check_resolved(&self, unit: &Unit) -> Result<(), Error> {
        let mut errs = vec![];

        for (_, off) in unit.labels.iter() {
            match off {
                LabelOffset::Resolved(_) => {}
                LabelOffset::Unresolved(positions) => {
                    for (ident, _) in positions.iter() {
                        errs.push(raise::not_declared::<CompileResult>(ident).err().unwrap());
                    }
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
