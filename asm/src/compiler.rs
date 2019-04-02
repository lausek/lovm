pub use super::*;

use self::parser::Ast;

use lovm::value::Value;
use std::collections::HashMap;
use std::str::FromStr;

pub type CompileResult = Result<Program, String>;

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
    labels: HashMap<String, LabelOffset>,
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
        println!("{:?}", ast);

        for step in ast.into_iter() {
            match step {
                Ast::Label(label) => self.declare_label(label, self.codeblock.len())?,
                Ast::Instruction(inx) => self.codeblock.push(Code::Instruction(inx)),
                Ast::Instruction1(inx, x1) => {
                    self.codeblock.push(Code::Instruction(inx));
                    self.compile_operand(x1)?;
                }
                Ast::Instruction2(inx, x1, x2) => {
                    self.codeblock.push(Code::Instruction(inx));
                    self.compile_operand(x1)?;
                    self.compile_operand(x2)?;
                }
            }
        }

        let labels = self
            .labels
            .iter()
            .map(|(ident, loff)| match loff {
                LabelOffset::Resolved(off) => Ok((ident.clone(), *off)),
                _ => Err(format!("label `{}` was not declared", ident)),
            })
            .collect::<Result<Vec<_>, String>>()?;

        let mut program = Program::with_code(self.codeblock);
        *program.labels_mut() = labels;

        Ok(program)
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
            Operand::Value(raw) => match Value::from_str(&raw) {
                Ok(value) => code = Code::Value(value),
                Err(msg) => return Err(msg),
            },
        }

        self.codeblock.push(code);
        Ok(())
    }

    fn declare_label(&mut self, label: String, off: usize) -> Result<(), String> {
        match self
            .labels
            .insert(label.clone(), LabelOffset::Resolved(off))
        {
            Some(LabelOffset::Resolved(_)) => Err(format!("redeclaration of label `{}`", label)),
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
}
