pub use super::*;

use self::parser::Ast;

use lovm::value::Value;
use std::collections::HashMap;
use std::str::FromStr;

pub type CompileResult = Result<Program, String>;

pub struct Compiler {
    labels: HashMap<String, usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
        }
    }

    pub fn compile(&mut self, src: &str) -> CompileResult {
        let ast = parser::parse(src)?;
        println!("{:?}", ast);

        let mut codeblock = CodeBlock::new();
        for step in ast.into_iter() {
            match step {
                Ast::Label(label) => {
                    let off = if codeblock.is_empty() {
                        0
                    } else {
                        codeblock.len()
                    };
                    self.labels.insert(label, off);
                }
                Ast::Instruction(inx) => codeblock.push(Code::Instruction(inx)),
                Ast::Instruction1(inx, x1) => {
                    codeblock.push(Code::Instruction(inx));
                    codeblock.push(self.compile_operand(x1)?);
                }
                Ast::Instruction2(inx, x1, x2) => {
                    codeblock.push(Code::Instruction(inx));
                    codeblock.push(self.compile_operand(x1)?);
                    codeblock.push(self.compile_operand(x2)?);
                }
            }
        }

        let mut program = Program::with_code(codeblock);
        *program.labels_mut() = self.labels.clone();

        Ok(program)
    }

    fn compile_operand(&self, op: Operand) -> Result<Code, String> {
        match op {
            // TODO: this doesn't allow for jump lookahead. whole code
            // must be generated in first place and addresses inserted afterwards.
            Operand::Ident(ident) => match self.labels.get(&ident) {
                Some(off) => Ok(Code::Ref(*off)),
                _ => Err(format!("label `{}` was not declared", ident)),
            },
            Operand::Register(reg) => Ok(Code::Register(reg)),
            Operand::Value(raw) => match Value::from_str(&raw) {
                Ok(value) => Ok(Code::Value(value)),
                Err(msg) => Err(msg),
            },
        }
    }
}
