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

#[derive(Debug, PartialEq, Eq)]
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
    pub fn id(&self) -> TokenId {
        match self {
            Self::Reserved(r) => match r {
                Reserved::Begin => TokenId::Begin,
                Reserved::End => TokenId::End,
            },
            Self::Ident(_) => TokenId::Ident,
            Self::IntLit(_) => TokenId::IntLit,
            Self::SemiColon => TokenId::SemiColon,
            Self::Operator(o) => match o {
                Op::Equal => TokenId::Equal,
                Op::Plus => TokenId::Plus,
                Op::Minus => TokenId::Minus,
            },
            Self::Unhandled(_) => TokenId::Unhandled,
            Self::WhiteSpace(_) => TokenId::WhiteSpace,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum TokenId {
    Begin,
    End,
    Ident,
    IntLit,
    SemiColon,
    Equal,
    Plus,
    Minus,
    Unhandled,
    WhiteSpace,
}
