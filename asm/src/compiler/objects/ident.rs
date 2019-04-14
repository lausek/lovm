use super::*;

#[derive(Clone, Debug, Eq, Default)]
pub struct Ident {
    pub(crate) loc: Location,
    pub(crate) raw: String,
}

impl Ident {
    pub fn new(loc: Location, raw: String) -> Self {
        Self { loc, raw }
    }

    pub fn is_register(&self) -> bool {
        use std::str::FromStr;
        Register::from_str(&self.raw).is_ok()
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.raw)
    }
}

impl std::cmp::PartialEq for Ident {
    fn eq(&self, other: &Ident) -> bool {
        self.raw == other.raw
    }
}

impl std::hash::Hash for Ident {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        self.raw.hash(h)
    }
}
