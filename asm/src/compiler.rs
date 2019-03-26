pub use super::*;

use self::parser::Ast;

pub type CompileResult = Result<Program, String>;

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&mut self, src: &str) -> CompileResult {
        let ast = parser::parse(src)?;
        println!("{:?}", ast);

        let mut codeblock = CodeBlock::new();
        for step in ast.into_iter() {
            match step {
                Ast::Instruction(inx) => codeblock.push(Code::Instruction(inx)),
                _ => unimplemented!(),
            }
        }

        Ok(Program::with_code(codeblock))
    }
}
