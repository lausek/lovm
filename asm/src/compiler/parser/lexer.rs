pub use super::*;

use std::str::FromStr;

pub type Tokens = Vec<Token>;
pub type Location = (usize, usize, usize);

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Ident(Ident),
    Keyword(Keyword),
    Str(String),
    Punct(char),
    SoftPunct,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub(crate) loc: Location,
    pub(crate) ty: TokenType,
}

impl Token {
    pub fn new(loc: Location, src: &str) -> Self {
        let ty = match Keyword::from_str(src) {
            Ok(kw) => TokenType::Keyword(kw),
            // TODO: check if `Ident` is lowercase register name (a-d) => return new Register(_) variant then
            _ => TokenType::Ident(Ident::new(loc, src.to_string())),
        };
        Self { loc, ty }
    }
}

pub fn lex_line(ldx: usize, src: &str) -> Result<Tokens, Error> {
    let mut toks = Tokens::new();
    let mut loc = (ldx, 0, 1);

    if src.is_empty() {
        return Ok(toks);
    }

    let mut it = src.chars().peekable();
    while let Some(c) = it.next() {
        match c {
            // @ => type argument
            // : => label postfix
            // # => contant prefix
            // * => deref prefix
            // . => macro prefix
            // ... => punctuation
            '@' | ':' | '#' | '*' | '.' | ',' | ' ' => {
                if 0 < loc.2 - loc.1 - 1 {
                    let span = (loc.1, loc.2 - 1);
                    let buffer = &src[span.0..span.1].trim();
                    if !buffer.is_empty() {
                        let tok = Token::new((ldx, span.0, span.1), buffer);
                        toks.push(tok);
                    }
                    loc.1 = span.1;
                }

                // whitespace isn't real punctuation
                if c == ' ' {
                    toks.push(Token {
                        loc,
                        ty: TokenType::SoftPunct,
                    });
                } else {
                    toks.push(Token {
                        loc,
                        ty: TokenType::Punct(c),
                    });
                }

                loc.1 = loc.2;
            }
            '"' => {
                // TODO: add location
                let s = take_string(&mut it)?;
                toks.push(Token {
                    loc: (0, 0, 0),
                    ty: TokenType::Str(s),
                });
            }
            // no punctuation char detected -> just expand buffer
            _ => {}
        }
        loc.2 += 1;
    }

    loc.2 -= 1;

    // TODO: trim probably not needed
    let buffer = &src[loc.1..loc.2].trim();
    if !buffer.is_empty() {
        let tok = Token::new(loc, buffer);
        toks.push(tok);
    }

    merge_softpunct(&mut toks);

    Ok(toks)
}

fn merge_softpunct(toks: &mut Tokens) {
    let mut last_soft: Option<Token> = None;
    let mut new_toks = vec![];

    for t in toks.drain(..) {
        match t {
            Token {
                ty: TokenType::SoftPunct,
                loc,
                ..
            } => {
                if let Some(last_soft) = &mut last_soft {
                    last_soft.loc.2 = loc.2;
                } else {
                    last_soft = Some(t);
                }
            }
            _ => {
                if let Some(last_soft) = last_soft.take() {
                    new_toks.push(last_soft);
                }
                new_toks.push(t);
            }
        }
    }

    *toks = new_toks;
}

fn take_string<T>(it: &mut std::iter::Peekable<T>) -> Result<String, Error>
where
    T: Iterator<Item = char>,
{
    let mut s = String::new();
    let mut escaped = false;
    while let Some(c) = it.next() {
        match c {
            '\\' if !escaped => {
                escaped = true;
                continue;
            }
            '"' if !escaped => return Ok(s),
            _ => s.push(c),
        }
        escaped = false;
    }
    raise::unclosed_string()
}
