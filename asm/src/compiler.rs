pub use super::*;

pub type CompileResult = Result<CodeBlock, String>;

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&mut self, src: &str) -> CompileResult {
        let ast = parser::parse(src);

        println!("{:?}", ast);

        Ok(CodeBlock::new())
    }
}
