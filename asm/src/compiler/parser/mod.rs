mod ident;
mod keyword;
mod lexer;

pub use self::ident::*;
pub use self::keyword::*;
pub use self::lexer::*;
pub use super::*;

pub type ParseResult = Result<Vec<Ast>, Error>;

#[derive(Clone, Debug)]
pub enum Ast {
    Label(Ident),
    Declare(String),
    Statement(Keyword),
    Statement1(Keyword, Operand),
    Statement2(Keyword, Operand, Operand),
}

#[derive(Clone, Debug)]
pub enum Operand {
    Deref(Box<Operand>),
    Register(Register),
    Value(lovm::value::Value),
    Ident(Ident),
}

pub fn parse(src: &str) -> ParseResult {
    let mut ls = vec![];

    for (ldx, line) in src.lines().enumerate() {
        let line = line.split(';').next().unwrap();
        if line.is_empty() {
            continue;
        }

        let tokens = lexer::lex_line(ldx, &line);
        if tokens.is_empty() {
            continue;
        }

        let inx = into_ast(tokens)?;
        ls.extend(inx);
    }

    Ok(ls)
}

fn into_ast(tokens: Tokens) -> Result<Vec<Ast>, Error> {
    let mut it = tokens.into_iter().peekable();
    match it.next() {
        Some(Token {
            ty: TokenType::Keyword(kw),
            ..
        }) => into_statement(kw, &mut it).and_then(|ast| Ok(vec![ast])),
        Some(Token {
            ty: TokenType::Ident(ident),
            ..
        }) => {
            let mut bl = vec![Ast::Label(ident)];
            expect(&mut it, TokenType::Punct(':'))?;

            match it.collect::<Vec<_>>() {
                tokens if !tokens.is_empty() => bl.extend(into_ast(tokens)?),
                _ => {}
            }

            Ok(bl)
        }
        _ => Err("line does not start with instruction".to_string().into()),
    }
}

fn into_statement<T>(kw: Keyword, it: &mut std::iter::Peekable<T>) -> Result<Ast, Error>
where
    T: Iterator<Item = Token>,
{
    match kw.arguments() {
        2 if kw == Keyword::Mov => {
            let indirect = take_deref(it);
            let mut to = take_op(it)?;
            if indirect {
                to = Operand::Deref(Box::new(to));
            }

            expect(it, TokenType::Punct(','))?;

            let indirect = take_deref(it);
            let mut from = take_op(it)?;
            if indirect {
                from = Operand::Deref(Box::new(from));
            }

            Ok(Ast::Statement2(kw, to, from))
        }
        2 => {
            let x1 = take_op(it)?;
            expect(it, TokenType::Punct(','))?;
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
    T: Iterator<Item = Token>,
{
    match it.peek() {
        Some(Token {
            ty: TokenType::Punct('*'),
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
    T: Iterator<Item = Token>,
{
    match it.next() {
        Some(Token {
            ty: TokenType::Punct('#'),
            ..
        }) => match it.next() {
            Some(Token {
                ty: TokenType::Ident(ident),
                ..
            }) => {
                use std::str::FromStr;
                let value = lovm::value::Value::from_str(&ident.raw)?;
                Ok(Operand::Value(value))
            }
            _ => Err("expected constant value".into()),
        },
        Some(Token {
            ty: TokenType::Ident(ident),
            ..
        }) => match ident.raw.as_ref() {
            "A" => Ok(Operand::Register(Register::A)),
            "B" => Ok(Operand::Register(Register::B)),
            "C" => Ok(Operand::Register(Register::C)),
            "D" => Ok(Operand::Register(Register::D)),
            _ => Ok(Operand::Ident(ident)),
        },
        what => Err(format!("unexpected token `{:?}`", what)),
    }
}

fn expect<T>(it: &mut T, expc: TokenType) -> Result<(), Error>
where
    T: Iterator<Item = Token>,
{
    match it.next() {
        Some(got) if got.ty == expc => Ok(()),
        got => raise::expected_got(expc, got.or(it.last())),
    }
}
