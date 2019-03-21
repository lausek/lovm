pub use super::*;

pub type CompileResult = Result<CodeBlock, String>;

pub fn compile(src: &str) -> CompileResult {
    let lex = lexer::lex(src);
    println!("{:?}", lex);
    Ok(CodeBlock::new())
}
