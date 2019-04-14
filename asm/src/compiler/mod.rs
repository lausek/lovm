pub use super::*;

pub mod error;
mod parser;
mod unit;

pub use self::error::*;
pub use self::parser::*;
pub use self::unit::*;

use self::parser::{Ast, Keyword};

use lovm::value::Value;
use std::collections::HashMap;
pub type CompileResult = Result<Program, Error>;

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
    Unresolved(Vec<usize>),
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
                Ast::Statement(kw) => unit.codeblock.push(Code::Instruction(kw.into_inx())),
                Ast::Statement1(kw, x1) if kw == Keyword::Dv => {
                    if let Operand::Value(value) = x1 {
                        unit.codeblock.push(Code::Value(value));
                    } else {
                        return raise::not_a_value(x1);
                    }
                }
                Ast::Statement1(kw, x1) => {
                    unit.codeblock.push(Code::Instruction(kw.into_inx()));
                    unit.compile_operand(x1)?;
                }
                Ast::Statement2(kw, x1, x2) if kw == Keyword::Mov => {
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
                    if let Operand::Deref(x2) = x2 {
                        unit.push_inx(Instruction::Push);
                        unit.compile_operand(*x2)?;
                        unit.push_inx(Instruction::Load);
                    } else {
                        unit.push_inx(Instruction::Push);
                        unit.compile_operand(x2)?;
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
                Ast::Statement2(kw, x1, x2) => match kw {
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
                        unit.push_inx(Instruction::Push);
                        unit.compile_operand(x1.clone())?;
                        unit.push_inx(Instruction::Push);
                        unit.compile_operand(x2)?;
                        unit.push_inx(kw.into_inx());
                        unit.push_inx(Instruction::Pop);
                        unit.compile_operand(x1)?;
                    }
                    _ => {
                        unit.push_inx(kw.into_inx());
                        unit.compile_operand(x1)?;
                        unit.compile_operand(x2)?;
                    }
                },
            }
        }

        let labels = unit
            .labels
            .iter()
            .map(|(ident, loff)| match loff {
                LabelOffset::Resolved(off) => Ok((ident.raw.clone(), *off)),
                _ => raise::not_declared(ident),
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let mut program = Program::with_code(unit.codeblock);
        *program.labels_mut() = labels;

        Ok(program)
    }
}
