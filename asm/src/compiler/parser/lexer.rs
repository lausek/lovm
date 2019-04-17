pub use super::*;

use std::str::FromStr;

pub type Tokens = Vec<Token>;
pub type Location = (usize, usize, usize);

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Ident(Ident),
    Keyword(Keyword),
    Punct(char),
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

pub fn lex_line(ldx: usize, src: &str) -> Tokens {
    let mut lex = Tokens::new();
    let mut loc = (ldx, 0, 1);

    if src.is_empty() {
        return lex;
    }

    for c in src.chars() {
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
                        lex.push(tok);
                    }
                }

                // whitespace isn't real punctuation
                if c != ' ' {
                    lex.push(Token {
                        loc,
                        ty: TokenType::Punct(c),
                    });
                }

                loc.1 = loc.2;
            }
            // no punctuation char detected -> just expand buffer
            _ => {}
        }
        loc.2 += 1;
    }

    loc.2 -= 1;

    let buffer = &src[loc.1..loc.2].trim();
    if !buffer.is_empty() {
        let tok = Token::new(loc, buffer);
        lex.push(tok);
    }

    lex
}
