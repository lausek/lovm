use super::*;

use colored::Colorize;

#[derive(Clone, Debug)]
pub struct Error {
    content: Vec<String>,
    loc: Location,
    ty: ErrorType,
}

#[derive(Clone, Debug)]
pub enum ErrorType {
    ExpectedGot,
    None,
    NotAValue,
    NotDeclared,
    Redeclared,
}

impl Error {
    pub fn new(ty: ErrorType) -> Self {
        Self {
            content: vec![],
            loc: (0, 0, 0),
            ty,
        }
    }

    pub fn msg(mut self, msg: String) -> Self {
        self.content
            .push(format!("{} {:?}: {}", "Err".red(), self.loc, msg));
        self
    }

    pub fn loc(mut self, loc: Location) -> Self {
        self.loc = loc;
        self
    }
}

impl std::convert::From<String> for Error {
    fn from(msg: String) -> Self {
        Self::new(ErrorType::None).msg(msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for line in self.content.iter() {
            write!(f, "{}", line)?;
        }
        Ok(())
    }
}

pub mod raise {
    use super::*;

    pub fn expected_got<T>(expc: LexTokenType, got: Option<LexToken>) -> Result<T, Error> {
        let mut err = Error::new(ErrorType::ExpectedGot);
        let got = if let Some(got) = got {
            err.loc = got.loc;
            format!("`{:?}`", got.ty)
        } else {
            "nothing".to_string()
        };
        let msg = format!("expected `{:?}`, got {}", expc, got);
        Err(err.msg(msg))
    }

    pub fn not_a_value<T>(raw: Operand) -> Result<T, Error> {
        Err(Error::new(ErrorType::NotAValue))
    }

    pub fn not_declared<T>(ident: &LexIdent) -> Result<T, Error> {
        let err = Error::new(ErrorType::NotDeclared).loc(ident.loc);
        let msg = format!("label `{}` was not declared", ident);
        Err(err.msg(msg))
    }

    pub fn redeclared<T>(ident: &LexIdent) -> Result<T, Error> {
        let msg = format!("redeclaration of label `{}`", ident);
        Err(Error::new(ErrorType::Redeclared).msg(msg))
    }
}
