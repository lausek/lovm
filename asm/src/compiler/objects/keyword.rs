pub use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
    Neg,
    And,
    Or,
    Xor,
    Shl,
    Shr,

    Cmp,
    Jmp,
    Jeq,
    Jne,
    Jge,
    Jgt,
    Jle,
    Jlt,

    Cast,
    Call,
    Int,
    Ret,
    Push,
    Pop,
    Pusha,
    Popa,

    // not really instructions
    Dv,
    Mov,
}

impl Keyword {
    pub fn arguments(&self) -> usize {
        match self {
            Keyword::Mov
            | Keyword::Add
            | Keyword::Sub
            | Keyword::Mul
            | Keyword::Div
            | Keyword::Rem
            | Keyword::Pow
            | Keyword::Neg
            | Keyword::And
            | Keyword::Or
            | Keyword::Xor
            | Keyword::Shl
            | Keyword::Shr
            | Keyword::Cmp
            | Keyword::Cast => 2,

            Keyword::Dv
            | Keyword::Inc
            | Keyword::Dec
            | Keyword::Jmp
            | Keyword::Jeq
            | Keyword::Jne
            | Keyword::Jge
            | Keyword::Jgt
            | Keyword::Jle
            | Keyword::Jlt
            | Keyword::Call
            | Keyword::Int
            | Keyword::Push
            | Keyword::Pop => 1,

            Keyword::Ret | Keyword::Pusha | Keyword::Popa => 0,
        }
    }
}

impl std::str::FromStr for Keyword {
    type Err = ();
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match from {
            "inc" => Ok(Keyword::Inc),
            "dec" => Ok(Keyword::Dec),
            "add" => Ok(Keyword::Add),
            "sub" => Ok(Keyword::Sub),
            "mul" => Ok(Keyword::Mul),
            "div" => Ok(Keyword::Div),
            "rem" => Ok(Keyword::Rem),
            "pow" => Ok(Keyword::Pow),
            "neg" => Ok(Keyword::Neg),
            "and" => Ok(Keyword::And),
            "or" => Ok(Keyword::Or),
            "xor" => Ok(Keyword::Xor),
            "shl" => Ok(Keyword::Shl),
            "shr" => Ok(Keyword::Shr),
            "cmp" => Ok(Keyword::Cmp),
            "jmp" => Ok(Keyword::Jmp),
            "jeq" => Ok(Keyword::Jeq),
            "jne" => Ok(Keyword::Jne),
            "jge" => Ok(Keyword::Jge),
            "jgt" => Ok(Keyword::Jgt),
            "jle" => Ok(Keyword::Jle),
            "jlt" => Ok(Keyword::Jlt),
            "call" => Ok(Keyword::Call),
            "cast" => Ok(Keyword::Cast),
            "int" => Ok(Keyword::Int),
            "ret" => Ok(Keyword::Ret),
            "push" => Ok(Keyword::Push),
            "pop" => Ok(Keyword::Pop),
            "pusha" => Ok(Keyword::Pusha),
            "popa" => Ok(Keyword::Popa),

            "dv" => Ok(Keyword::Dv),
            "mov" => Ok(Keyword::Mov),
            _ => Err(()),
        }
    }
}
