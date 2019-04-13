pub use super::*;

use self::parser::{Ast, Keyword};

use lovm::value::Value;
use std::collections::HashMap;
use std::str::FromStr;

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

pub struct Compiler {
    codeblock: CodeBlock,
    labels: HashMap<Ident, LabelOffset>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            codeblock: CodeBlock::new(),
            labels: HashMap::new(),
        }
    }

    pub fn compile(mut self, src: &str) -> CompileResult {
        let ast = parser::parse(src)?;
        //println!("{:?}", ast);

        for step in ast.into_iter() {
            match step {
                Ast::Label(ident) => self.declare_label(ident, self.codeblock.len())?,
                Ast::Declare(value) => self.declare_value(value)?,
                Ast::Statement(kw) => self.codeblock.push(Code::Instruction(kw.into_inx())),
                Ast::Statement1(kw, x1) if kw == Keyword::Dv => {
                    if let Operand::Value(value) = x1 {
                        self.codeblock.push(Code::Value(value));
                    } else {
                        return raise::not_a_value(x1);
                    }
                }
                Ast::Statement1(kw, x1) => {
                    self.codeblock.push(Code::Instruction(kw.into_inx()));
                    self.compile_operand(x1)?;
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
                        self.push_inx(Instruction::Push);
                        self.compile_operand(*x2)?;
                        self.push_inx(Instruction::Load);
                    } else {
                        self.push_inx(Instruction::Push);
                        self.compile_operand(x2)?;
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
                        self.push_inx(Instruction::Push);
                        self.compile_operand(x1.clone())?;
                        self.push_inx(Instruction::Push);
                        self.compile_operand(x2)?;
                        self.push_inx(kw.into_inx());
                        self.push_inx(Instruction::Pop);
                        self.compile_operand(x1)?;
                    }
                    _ => {
                        self.push_inx(kw.into_inx());
                        self.compile_operand(x1)?;
                        self.compile_operand(x2)?;
                    }
                },
            }
        }

        let labels = self
            .labels
            .iter()
            .map(|(ident, loff)| match loff {
                LabelOffset::Resolved(off) => Ok((ident.raw.clone(), *off)),
                _ => raise::not_declared(ident),
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let mut program = Program::with_code(self.codeblock);
        *program.labels_mut() = labels;

        Ok(program)
    }

    fn push_inx(&mut self, inx: Instruction) {
        self.codeblock.push(Code::Instruction(inx));
    }

    fn compile_operand(&mut self, op: Operand) -> Result<(), String> {
        // we have to push a placeholder value or the index will become corrupt
        let mut code = mkref(std::usize::MAX);

        match op {
            Operand::Ident(ident) => match self.labels.get_mut(&ident) {
                Some(LabelOffset::Resolved(off)) => code = mkref(*off),
                Some(LabelOffset::Unresolved(positions)) => positions.push(self.codeblock.len()),
                _ => {
                    self.labels
                        .insert(ident, LabelOffset::Unresolved(vec![self.codeblock.len()]));
                }
            },
            Operand::Register(reg) => code = Code::Register(reg),
            Operand::Value(value) => code = Code::Value(value),
            Operand::Deref(_) => unreachable!(),
        }

        self.codeblock.push(code);
        Ok(())
    }

    fn declare_label(&mut self, label: Ident, off: usize) -> Result<(), Error> {
        match self
            .labels
            .insert(label.clone(), LabelOffset::Resolved(off))
        {
            Some(LabelOffset::Resolved(_)) => raise::redeclared(&label),
            // use reverse order to not invalidate indices
            Some(LabelOffset::Unresolved(positions)) => {
                for pos in positions.into_iter().rev() {
                    *self.codeblock.get_mut(pos).unwrap() = mkref(off);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn declare_value(&mut self, value: String) -> Result<(), String> {
        let value = Value::from_str(&value)?;
        self.codeblock.push(Code::Value(value));
        Ok(())
    }
}
