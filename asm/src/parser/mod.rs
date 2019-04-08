mod lexer;

use self::lexer::*;
pub use super::*;

pub type ParseResult = Result<Vec<Ast>, String>;

#[derive(Clone, Debug)]
pub enum Ast {
    Label(String),
    Declare(String),
    Instruction(Instruction),
    Instruction1(Instruction, Operand),
    Instruction2(Instruction, Operand, Operand),
}

#[derive(Clone, Debug)]
pub enum Operand {
    Register(Register),
    Value(String),
    Ident(String),
}

pub fn parse(src: &str) -> ParseResult {
    let mut ls = vec![];

    for (_ldx, line) in src.lines().enumerate() {
        let line = line.split(';').next().unwrap();
        if line.is_empty() {
            continue;
        }

        let tokens = lexer::lex_line(&line);
        if tokens.is_empty() {
            continue;
        }

        println!("{:?}", tokens);

        let inx = into_ast(tokens)?;
        ls.push(inx);
    }

    Ok(ls)
}

fn into_ast(tokens: LexTokens) -> Result<Ast, String> {
    let mut it = tokens.into_iter().peekable();
    match it.next() {
        Some(LexToken {
            ty: LexTokenType::Instruction(inx),
            ..
        }) => into_instruction(inx, &mut it),
        Some(LexToken {
            ty: LexTokenType::Ident(label),
            ..
        }) => {
            expect(&mut it, LexTokenType::Punct(':'))?;
            Ok(Ast::Label(label))
        }
        Some(LexToken {
            ty: LexTokenType::Internal(inx),
            ..
        }) => match inx {
            InternalInstruction::Declare => {
                match take_op(&mut it)? {
                    Operand::Value(value) => Ok(Ast::Declare(value)),
                    _ => panic!("invalid operand for declare"),
                }
            }
        }
        _ => Err("line does not start with instruction".into()),
    }
}

fn into_instruction<T>(inx: Instruction, it: &mut std::iter::Peekable<T>) -> Result<Ast, String>
where
    T: Iterator<Item = LexToken>,
{
    match inx.arguments() {
        2 => {
            let x1_deref = if inx == Instruction::Mov {
                take_deref(it)
            } else {
                false
            };
            let x1 = take_op(it)?;

            expect(it, LexTokenType::Punct(','))?;

            let x2_deref = if inx == Instruction::Mov {
                take_deref(it)
            } else {
                false
            };
            let x2 = take_op(it)?;

            let inx = if inx == Instruction::Mov {
                match (x1_deref, x2_deref) {
                    (true, true) => Instruction::Copy,
                    (false, true) => Instruction::Load,
                    (true, false) => Instruction::Store,
                    _ => Instruction::Mov,
                }
            } else {
                inx
            };

            Ok(Ast::Instruction2(inx, x1, x2))
        }
        1 => {
            let x1 = take_op(it)?;
            Ok(Ast::Instruction1(inx, x1))
        }
        0 => Ok(Ast::Instruction(inx)),
        _ => unreachable!(),
    }
}

fn take_deref<T>(it: &mut std::iter::Peekable<T>) -> bool
where
    T: Iterator<Item = LexToken>,
{
    match it.peek() {
        Some(LexToken {
            ty: LexTokenType::Punct('*'),
            ..
        }) => {
            it.next().unwrap();
            true
        }
        _ => false,
    }
}

fn take_op<T>(it: &mut std::iter::Peekable<T>) -> Result<Operand, String>
where
    T: Iterator<Item = LexToken>,
{
    match it.next() {
        Some(LexToken {
            ty: LexTokenType::Punct('#'),
            ..
        }) => match it.next() {
            Some(LexToken {
                ty: LexTokenType::Ident(value),
                ..
            }) => Ok(Operand::Value(value)),
            _ => Err("expected constant value".into()),
        },
        Some(LexToken {
            ty: LexTokenType::Ident(ident),
            ..
        }) => match ident.as_ref() {
            "A" => Ok(Operand::Register(Register::A)),
            "B" => Ok(Operand::Register(Register::B)),
            "C" => Ok(Operand::Register(Register::C)),
            "D" => Ok(Operand::Register(Register::D)),
            _ => Ok(Operand::Ident(ident.clone())),
        },
        what => Err(format!("unexpected token `{:?}`", what)),
    }
}

fn expect<T>(it: &mut T, expc: LexTokenType) -> Result<(), String>
where
    T: Iterator<Item = LexToken>,
{
    match it.next() {
        Some(got) if got.ty == expc => Ok(()),
        got => Err(format!("expected `{:?}`, got `{:?}`", expc, got)),
    }
}
