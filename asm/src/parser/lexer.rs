pub use super::*;

use std::str::FromStr;

pub type LexTokens = Vec<LexToken>;
pub type Location = (usize, usize, usize);

#[derive(Clone, Debug, PartialEq)]
pub enum LexTokenType {
    Ident(LexIdent),
    Keyword(Keyword),
    Punct(char),
}

#[derive(Clone, Debug)]
pub struct LexToken {
    pub(crate) loc: Location,
    pub(crate) ty: LexTokenType,
}

#[derive(Clone, Debug, Eq)]
pub struct LexIdent {
    pub(crate) loc: Location,
    pub(crate) raw: String,
}

impl LexIdent {
    pub fn new(loc: Location, raw: String) -> Self {
        Self { loc, raw }
    }
}

impl std::fmt::Display for LexIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.raw)
    }
}

impl std::cmp::PartialEq for LexIdent {
    fn eq(&self, other: &LexIdent) -> bool {
        self.raw == other.raw
    }
}

impl std::hash::Hash for LexIdent {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        self.raw.hash(h)
    }
}

impl LexToken {
    pub fn new(loc: Location, src: &str) -> Self {
        let ty = match Keyword::from_str(src) {
            Ok(kw) => LexTokenType::Keyword(kw),
            // TODO: check if `Ident` is lowercase register name (a-d) => return new Register(_) variant then
            _ => LexTokenType::Ident(LexIdent::new(loc, src.to_string())),
        };
        Self { loc, ty }
    }
}

pub fn lex_line(ldx: usize, src: &str) -> LexTokens {
    let mut lex = LexTokens::new();
    let mut loc = (ldx, 0, 1);

    if src.is_empty() {
        return lex;
    }

    for c in src.chars() {
        match c {
            ':' | '#' | ',' | ' ' | '*' => {
                if 0 < loc.2 - loc.1 - 1 {
                    let span = (loc.1, loc.2 - 1);
                    let buffer = &src[span.0..span.1].trim();
                    if !buffer.is_empty() {
                        let tok = LexToken::new((ldx, span.0, span.1), buffer);
                        lex.push(tok);
                    }
                }

                // whitespace isn't real punctuation
                if c != ' ' {
                    lex.push(LexToken {
                        loc,
                        ty: LexTokenType::Punct(c),
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
        let tok = LexToken::new(loc, buffer);
        lex.push(tok);
    }

    lex
}
