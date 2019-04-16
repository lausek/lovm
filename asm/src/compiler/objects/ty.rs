#[derive(Clone, Debug)]
pub enum Type {
    I,
    I64,
    F64,
    Ref,
    T,
}

impl Into<i8> for Type {
    fn into(self) -> i8 {
        match self {
            Type::I => 1,
            Type::I64 => 2,
            Type::F64 => 3,
            Type::Ref => 4,
            Type::T => 5,
        }
    }
}

impl Into<usize> for Type {
    fn into(self) -> usize {
        let x: i8 = self.into();
        x as usize
    }
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
