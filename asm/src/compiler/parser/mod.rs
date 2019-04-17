mod lexer;

pub use self::lexer::*;

pub use super::*;

pub type ParseResult = Result<Vec<Ast>, Error>;

#[derive(Clone, Debug)]
pub enum Ast {
    Declare(String),
    Label(Ident),
    Macro(Ident, Vec<Operand>),
    Statement(Statement),
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
    take_softpunct(&mut it);
    match it.next() {
        Some(Token {
            ty: TokenType::Keyword(kw),
            ..
        }) => {
            take_softpunct(&mut it);
            into_statement(kw, &mut it).and_then(|ast| Ok(vec![ast]))
        }
        Some(Token {
            ty: TokenType::Punct('.'),
            ..
        }) => into_macro(&mut it).and_then(|ast| Ok(vec![ast])),
        Some(Token {
            ty: TokenType::Ident(ident),
            ..
        }) if !ident.is_register() => {
            let mut bl = vec![Ast::Label(ident)];
            expect(&mut it, TokenType::Punct(':'))?;
            take_softpunct(&mut it);

            match it.collect::<Vec<_>>() {
                tokens if !tokens.is_empty() => bl.extend(into_ast(tokens)?),
                _ => {}
            }

            Ok(bl)
        }
        what => raise::expected_either_got(&["label", "instruction"], what),
    }
}

fn into_statement<T>(kw: Keyword, it: &mut std::iter::Peekable<T>) -> Result<Ast, Error>
where
    T: Iterator<Item = Token>,
{
    let ty = take_type(it);

    take_softpunct(it);

    match kw.arguments() {
        2 if kw == Keyword::Mov => {
            let indirect = take_deref(it);
            let mut to = take_op(it)?;
            if indirect {
                to = Operand::Deref(Box::new(to));
            }

            take_separator(it)?;

            let indirect = take_deref(it);
            let mut from = take_op(it)?;
            if indirect {
                from = Operand::Deref(Box::new(from));
            }
            let stmt = Statement::from(kw, ty).arg(to).arg(from);
            Ok(Ast::Statement(stmt))
        }
        2 => {
            let x1 = take_op(it)?;
            take_separator(it)?;
            let x2 = take_op(it)?;
            let stmt = Statement::from(kw, ty).arg(x1).arg(x2);
            Ok(Ast::Statement(stmt))
        }
        1 => {
            let stmt = Statement::from(kw, ty).arg(take_op(it)?);
            Ok(Ast::Statement(stmt))
        }
        0 => Ok(Ast::Statement(Statement::from(kw.into(), ty))),
        _ => unreachable!(),
    }
}

fn into_macro<T>(it: &mut std::iter::Peekable<T>) -> Result<Ast, Error>
where
    T: Iterator<Item = Token>,
{
    match it.next() {
        Some(Token {
            ty: TokenType::Ident(ident),
            ..
        }) => {
            take_softpunct(it);
            Ok(Ast::Macro(ident.clone(), take_ops(it)?))
        }
        got => raise::expected_either_got(&vec!["label"], got),
    }
}

fn take_separator<T>(it: &mut std::iter::Peekable<T>) -> Result<(), Error>
where
    T: Iterator<Item = Token>,
{
    take_softpunct(it);
    expect(it, TokenType::Punct(','))?;
    take_softpunct(it);
    Ok(())
}

fn take_softpunct<T>(it: &mut std::iter::Peekable<T>)
where
    T: Iterator<Item = Token>,
{
    if let Some(Token {
        ty: TokenType::SoftPunct,
        ..
    }) = it.peek()
    {
        it.next().unwrap();
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

fn take_ops<T>(it: &mut std::iter::Peekable<T>) -> Result<Vec<Operand>, String>
where
    T: Iterator<Item = Token>,
{
    let mut ops = vec![];
    loop {
        ops.push(take_op(it)?);
        take_softpunct(it);
        match it.peek() {
            Some(Token {
                ty: TokenType::Punct(','),
                ..
            }) => {
                take_softpunct(it);
            }
            _ => break,
        }
    }
    Ok(ops)
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

fn take_type<T>(it: &mut std::iter::Peekable<T>) -> Option<Type>
where
    T: Iterator<Item = Token>,
{
    match it.peek() {
        Some(Token {
            ty: TokenType::Punct('@'),
            ..
        }) => {
            use std::str::FromStr;
            it.next().unwrap();
            match it.next() {
                Some(Token {
                    ty: TokenType::Ident(ident),
                    ..
                }) => match Type::from_str(&ident.raw) {
                    Ok(ty) => Some(ty),
                    _ => None,
                },
                // TODO: should actually raise an error: expect_either_got
                _ => unimplemented!(),
            }
        }
        _ => None,
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
