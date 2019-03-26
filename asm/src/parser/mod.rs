mod lexer;

pub use super::*;

pub type ParseResult = Result<Vec<Ast>, String>;

#[derive(Clone, Debug)]
pub enum Ast {
    Instruction(Instruction),
    Instruction1(Instruction, Operand),
    Instruction2(Instruction, Operand, Operand),
}

#[derive(Clone, Debug)]
pub enum Operand {
    Register(Register),
    Value,
    Ident(String),
}

pub fn parse(src: &str) -> ParseResult {
    let mut ls = vec![];

    for (ldx, line) in src.lines().enumerate() {
        let line = line.split(';').next().unwrap();
        if line.is_empty() {
            continue;
        }

        let tokens = lexer::lex_line(&line);
        if tokens.is_empty() {
            continue;
        }

        let inx = to_instruction(tokens)?;
        ls.push(inx);
    }

    Ok(ls)
}

fn to_instruction(tokens: lexer::LexTokens) -> Result<Ast, String> {
    match tokens.get(0) {
        Some(lexer::LexToken {
            ty: lexer::LexTokenType::Instruction(inx),
            ..
        }) => {
            let argc = inx.arguments();
            Ok(Ast::Instruction(*inx))
        }
        _ => Err("line does not start with instruction".into()),
    }
}
