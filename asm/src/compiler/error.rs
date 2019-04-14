use super::*;

use colored::Colorize;

#[derive(Clone, Debug)]
pub struct Error {
    // TODO: should include other `Error`s aswell
    content: Vec<String>,
    loc: Location,
    // TODO: should rather be an option
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

impl std::convert::From<Vec<Self>> for Error {
    fn from(errs: Vec<Self>) -> Self {
        let mut new = Self::new(ErrorType::None);
        for err in errs {
            new.content.extend(err.content);
        }
        new
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
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

pub mod raise {
    use super::*;

    pub fn expected_either_got<T>(expc: &[&str], got: Option<Token>) -> Result<T, Error> {
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

    pub fn expected_got<T>(expc: TokenType, got: Option<Token>) -> Result<T, Error> {
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

    pub fn not_a_value<T>(op: Operand) -> Result<T, Error> {
        let err = Error::new(ErrorType::NotAValue);
        let msg = format!("`{:?}` cannot be interpreted as value", op);
        Err(err.msg(msg))
    }

    pub fn not_declared<T>(ident: &Ident) -> Result<T, Error> {
        let err = Error::new(ErrorType::NotDeclared).loc(ident.loc);
        let msg = format!("label `{}` was not declared", ident);
        Err(err.msg(msg))
    }

    pub fn redeclared<T>(ident: &Ident) -> Result<T, Error> {
        let msg = format!("redeclaration of label `{}`", ident);
        Err(Error::new(ErrorType::Redeclared).msg(msg))
    }
}
