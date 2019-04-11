mod keyword;
mod lexer;

pub use self::keyword::*;
pub use self::lexer::*;
pub use super::*;

pub type ParseResult = Result<Vec<Ast>, String>;

#[derive(Clone, Debug)]
pub enum Ast {
    Label(String),
    Declare(String),
    Statement(Keyword),
    Statement1(Keyword, Operand),
    Statement2(Keyword, Operand, Operand),
}

#[derive(Clone, Debug)]
pub enum Operand {
    Deref(Box<Operand>),
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
        ls.extend(inx);
    }

    Ok(ls)
}

fn into_ast(tokens: LexTokens) -> Result<Vec<Ast>, String> {
    let mut it = tokens.into_iter().peekable();
    match it.next() {
        Some(LexToken {
            ty: LexTokenType::Keyword(kw),
            ..
        }) => into_statement(kw, &mut it).and_then(|ast| Ok(vec![ast])),
        Some(LexToken {
            ty: LexTokenType::Ident(label),
            ..
        }) => {
            let mut bl = vec![Ast::Label(label)];
            expect(&mut it, LexTokenType::Punct(':'))?;

            match it.collect::<Vec<_>>() {
                tokens if !tokens.is_empty() => bl.extend(into_ast(tokens)?),
                _ => {}
            }

            Ok(bl)
        }
        _ => Err("line does not start with instruction".into()),
    }
}

fn into_statement<T>(kw: Keyword, it: &mut std::iter::Peekable<T>) -> Result<Ast, String>
where
    T: Iterator<Item = LexToken>,
{
    match kw.arguments() {
        2 if kw == Keyword::Mov => {
            let indirect = take_deref(it);
            let mut to = take_op(it)?;
            if indirect {
                to = Operand::Deref(Box::new(to));
            }

            expect(it, LexTokenType::Punct(','))?;

            let indirect = take_deref(it);
            let mut from = take_op(it)?;
            if indirect {
                from = Operand::Deref(Box::new(from));
            }

            Ok(Ast::Statement2(kw, to, from))
        }
        2 => {
            let x1 = take_op(it)?;
            expect(it, LexTokenType::Punct(','))?;
            let x2 = take_op(it)?;
            Ok(Ast::Statement2(kw, x1, x2))
        }
        1 => Ok(Ast::Statement1(kw, take_op(it)?)),
        0 => Ok(Ast::Statement(kw)),
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
