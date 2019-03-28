use lovm::code::*;

use std::str::FromStr;

pub type LexTokens = Vec<LexToken>;
pub type Location = (usize, usize);

#[derive(Clone, Debug, PartialEq)]
pub enum LexTokenType {
    Ident(String),
    Instruction(Instruction),
    Punct(char),
}

#[derive(Clone, Debug)]
pub struct LexToken {
    pub(crate) loc: Location,
    pub(crate) ty: LexTokenType,
}

impl LexToken {
    pub fn new(loc: Location, src: &str) -> Self {
        Self {
            loc,
            ty: to_type(src),
        }
    }
}

pub fn lex_line(src: &str) -> LexTokens {
    let mut lex = LexTokens::new();
    let mut loc = (0, 1);

    if src.is_empty() {
        return lex;
    }

    for c in src.chars() {
        match c {
            ':' | '#' | ',' | ' ' => {
                if 0 < loc.1 - loc.0 - 1 {
                    let span = (loc.0, loc.1 - 1);
                    let buffer = &src[span.0..span.1].trim();
                    if !buffer.is_empty() {
                        let tok = LexToken::new(span, buffer);
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

                loc.0 = loc.1;
            }
            // no punctuation char detected -> just expand buffer
            _ => {}
        }
        loc.1 += 1;
    }

    loc.1 -= 1;

    let buffer = &src[loc.0..loc.1].trim();
    if !buffer.is_empty() {
        let tok = LexToken::new(loc, buffer);
        lex.push(tok);
    }

    lex
}

fn to_type(buffer: &str) -> LexTokenType {
    if let Ok(inx) = Instruction::from_str(buffer) {
        return LexTokenType::Instruction(inx);
    }
    // TODO: check if `Ident` is lowercase register name (a-d) => return new Register(_) variant then
    LexTokenType::Ident(buffer.to_string())
}
