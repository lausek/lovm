#[derive(Clone, Debug)]
pub enum Type {
    I,
    I64,
    F64,
    Ref,
    T,
}

impl std::str::FromStr for Type {
    type Err = ();
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match from {
            "i" => Ok(Type::I),
            "i64" => Ok(Type::I64),
            "f64" => Ok(Type::F64),
            "ref" => Ok(Type::Ref),
            "t" => Ok(Type::T),
            _ => Err(()),
        }
    }
}
