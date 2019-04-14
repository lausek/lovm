use super::*;

#[derive(Clone, Debug)]
pub struct Statement {
    pub(crate) kw: Keyword,
    pub(crate) ty: Option<Type>,
    pub(crate) arg1: Option<Operand>,
    pub(crate) arg2: Option<Operand>,
}

impl Statement {
    pub fn from(kw: Keyword, ty: Option<Type>) -> Self {
        Self {
            kw,
            ty,
            arg1: None,
            arg2: None,
        }
    }

    pub fn ty(mut self, ty: Option<Type>) -> Self {
        self.ty = ty;
        self
    }

    pub fn arg1(mut self, arg1: Operand) -> Self {
        self.arg1 = Some(arg1);
        self
    }

    pub fn arg2(mut self, arg2: Operand) -> Self {
        self.arg2 = Some(arg2);
        self
    }
}

impl Statement {
    pub fn inx(&self) -> Instruction {
        match self.kw {
            Keyword::Inc => Instruction::Inc,
            Keyword::Dec => Instruction::Dec,
            Keyword::Add => Instruction::Add,
            Keyword::Sub => Instruction::Sub,
            Keyword::Mul => Instruction::Mul,
            Keyword::Div => Instruction::Div,
            Keyword::Rem => Instruction::Rem,
            Keyword::Pow => Instruction::Pow,
            Keyword::Neg => Instruction::Neg,
            Keyword::And => Instruction::And,
            Keyword::Or => Instruction::Or,
            Keyword::Xor => Instruction::Xor,
            Keyword::Shl => Instruction::Shl,
            Keyword::Shr => Instruction::Shr,
            Keyword::Cmp => Instruction::Cmp,
            Keyword::Jmp => Instruction::Jmp,
            Keyword::Jeq => Instruction::Jeq,
            Keyword::Jne => Instruction::Jne,
            Keyword::Jge => Instruction::Jge,
            Keyword::Jgt => Instruction::Jgt,
            Keyword::Jle => Instruction::Jle,
            Keyword::Jlt => Instruction::Jlt,
            Keyword::Coal => Instruction::Coal,
            Keyword::Call => Instruction::Call,
            Keyword::Ret => Instruction::Ret,
            Keyword::Push => Instruction::Push,
            Keyword::Pop => Instruction::Pop,
            Keyword::Pusha => Instruction::Pusha,
            Keyword::Popa => Instruction::Popa,
            _ => panic!("not an instruction"),
        }
    }
}
