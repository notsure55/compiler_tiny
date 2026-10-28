#[derive(Debug, PartialEq, Eq)]
pub enum Reserved {
    Begin,
    End,
}

impl Reserved {
    pub fn new(r: &str) -> Option<Self> {
        match r {
            "BEGIN" => Some(Reserved::Begin),
            "END" => Some(Reserved::End),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Equal,
    Plus,
    Minus,
}

#[derive(Debug)]
pub enum Token<'a> {
    Reserved(Reserved),
    Ident(&'a str),
    IntLit(i32),
    SemiColon,
    Operator(Op),
    Unhandled(&'a str),
    WhiteSpace(&'a str),
}

impl<'a> Token<'a> {
    pub fn reserved(&self) -> Option<Self> {
        match self {
            Self::Ident(s) => Some(Token::Reserved(Reserved::new(s)?)),
            _ => None,
        }
    }
}
