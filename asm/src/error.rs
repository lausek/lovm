use super::*;

#[derive(Clone, Debug)]
pub struct Error {
    args: Vec<String>,
    ty: ErrorType,
}

#[derive(Clone, Debug)]
pub enum ErrorType {
    NotAValue,
    NotDeclared,
}

impl Error {
    pub fn new() -> Self {
        Self {
            args: vec![],
            ty: ErrorType::NotAValue,
        }
    }

    pub fn raise(ty: ErrorType, args: Vec<String>) -> Self {
        Self { args, ty }
    }
}

impl std::convert::From<String> for Error {
    fn from(_msg: String) -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self.ty {
            ErrorType::NotAValue => write!(f, "{:?} is not a value", self.args),
            ErrorType::NotDeclared => write!(f, "label {:?} was not declared", self.args),
        }
    }
}
