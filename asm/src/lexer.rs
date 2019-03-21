pub type Location = (usize, usize, usize);

#[derive(Clone, Debug)]
pub struct LexToken {
    loc: Location,
    tag: LexTokenTag,
}

#[derive(Clone, Copy, Debug)]
pub enum LexTokenTag {
    RegA,
    RegB,
    RegC,
    RegD,

    Label,
    Ident,
    Numc,
    Logc,

    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
    And,
    Or,
    Xor,

    Cmp,
    Jmp,
    Jeq,
    Jne,
    Jge,
    Jgt,
    Jle,
    Jlt,

    Mov,
    Call,
    Ret,
    Push,
    Pop,
    Pusha,
    Popa,
}

#[derive(Debug)]
pub struct Lex {
    pub(crate) src: String,
    pub(crate) tokens: Vec<LexToken>,
}

impl Lex {
    pub fn new(src: String) -> Self {
        Self {
            src,
            tokens: vec![],
        }
    }

    pub fn read(&self, loc: Location) -> Option<&str> {
        self.src.lines().nth(loc.0).map(|line| &line[loc.1..loc.2])
    }
}

pub fn lex(src: &str) -> Lex {
    let mut lex = Lex::new(src.to_string());

    for (ldx, line) in lex.src.lines().enumerate() {
        let line = line.split(';').next().unwrap();
        let mut loc = (ldx, 0, 1);

        for c in line.chars() {
            println!("{} at {:?}: {:?}", c, loc, lex.tokens);
            match c {
                ',' | ' ' if 0 < loc.2 - loc.1 - 1 => {
                    let tag = match_tag(&line[loc.1..(loc.2 - 1)]);
                    lex.tokens.push(LexToken { loc, tag });
                    loc.1 = loc.2;
                }
                _ => {}
            }
            loc.2 += 1;
        }

        loc.2 -= 1;
        let tag = match_tag(&line[loc.1..loc.2]);
        lex.tokens.push(LexToken { loc, tag });
    }

    lex
}

fn match_tag(buffer: &str) -> LexTokenTag {
    let buffer = buffer.trim();
    if buffer.starts_with("#") {
        LexTokenTag::Numc
    } else if buffer.ends_with(":") {
        LexTokenTag::Label
    } else {
        match buffer {
            "A" => LexTokenTag::RegA,
            "B" => LexTokenTag::RegB,
            "C" => LexTokenTag::RegC,
            "D" => LexTokenTag::RegD,

            "inc" => LexTokenTag::RegD,
            "dec" => LexTokenTag::Dec,
            "add" => LexTokenTag::Add,
            "sub" => LexTokenTag::Sub,
            "mul" => LexTokenTag::Mul,
            "div" => LexTokenTag::Div,
            "rem" => LexTokenTag::Rem,
            "pow" => LexTokenTag::Pow,
            "and" => LexTokenTag::And,
            "or" => LexTokenTag::Or,
            "xor" => LexTokenTag::Xor,
            "cmp" => LexTokenTag::Cmp,
            "jmp" => LexTokenTag::Jmp,
            "jeq" => LexTokenTag::Jeq,
            "jne" => LexTokenTag::Jne,
            "jge" => LexTokenTag::Jge,
            "jgt" => LexTokenTag::Jgt,
            "jle" => LexTokenTag::Jle,
            "jlt" => LexTokenTag::Jlt,
            "mov" => LexTokenTag::Mov,
            "call" => LexTokenTag::Call,
            "ret" => LexTokenTag::Ret,
            "push" => LexTokenTag::Push,
            "pop" => LexTokenTag::Pop,
            "pusha" => LexTokenTag::Pusha,
            "popa" => LexTokenTag::Popa,
            _ => panic!("unknown lex token `{:?}`", buffer),
        }
    }
}
