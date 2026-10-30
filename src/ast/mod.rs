use super::token::*;
use anyhow::{anyhow, Result};
use std::iter::Peekable;

pub mod production;

#[derive(Debug)]
pub enum AstNode {
    Var(Var),
    Constant(Constant),
    Operator(Operator),
    Unknown,
}

impl AstNode {
    pub fn new<'a>(token: Token<'a>) -> Option<Self> {
        let node = match token {
            Token::IntLit(i) => AstNode::Constant(Constant::new(i)),
            Token::Ident(i) => AstNode::Var(Var::new(i)),
            Token::Operator(o) => AstNode::Operator(Operator::new(o)),
            _ => todo!("Handle this case!"),
        };

        Some(node)
    }
}

// TODO: can add a type for the info, like float, int or whatevs
// TODO: operator precedence when adding new ops like *
#[derive(Debug)]
pub struct Operator {
    op: Op,
    info: Info,
}

impl Operator {
    pub fn new(op: Op) -> Self {
        Self {
            op,
            info: Info::default(),
        }
    }
}
#[derive(Debug)]
pub struct Constant {
    value: i32,
    info: Info,
}

impl Constant {
    pub fn from_token(token: Token) -> Option<Self> {
        if let Token::IntLit(value) = token {
            Some(Self {
                value,
                info: Info::default(),
            })
        } else {
            None
        }
    }
    pub fn new(value: i32) -> Self {
        Self {
            value,
            info: Info::default(),
        }
    }
}
#[derive(Debug)]
pub struct Var {
    name: String,
    info: Info,
}

impl Var {
    pub fn new(ident: &str) -> Self {
        Self {
            name: ident.to_owned(),
            info: Info::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Info {
    line: u32,
    col: u32,
}

#[derive(Debug)]
pub struct Expr {
    left: Box<AstNode>,
    op: Option<Operator>,
    right: Option<Box<AstNode>>,
}

impl Expr {
    pub fn new(tokens: &[Token]) -> Option<Self> {
        None
    }
    pub fn from_nodes(mut nodes: impl Iterator<Item = AstNode>) -> Option<Self> {
        None
    }
}

#[derive(Debug)]
pub enum Ast {
    Assign(Assign),
    Expr(Expr),
}

impl Ast {
    pub fn nodes(&self) -> Vec<&AstNode> {
        match self {
            Self::Assign(a) => {
                vec![]
            }
            _ => {
                vec![]
            }
        }
    }
    fn parse_ast<'a, I: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<I>) -> Result<Self> {
        Ok(Ast::Assign(Assign::parse_ast(tokens).unwrap()))
    }
}

#[derive(Debug)]
pub struct Assign {
    left: AstNode,
    right: Expr,
}

impl Assign {
    pub fn parse_ast<'a, I: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<I>) -> Option<Self> {
        //match tokens.peek() {}
        None
    }
}

#[derive(Debug)]
pub struct StatementSeq {
    stmt: Ast,
    next: Option<Box<Self>>,
}

impl StatementSeq {
    pub fn new(stmt: Ast, next: Option<Box<Self>>) -> Self {
        Self { stmt, next }
    }
    pub fn from_tokens<'a, I: Iterator<Item = Token<'a>>>(
        tokens: &mut Peekable<I>,
    ) -> Option<Self> {
        let ast = match Ast::parse_ast(tokens) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{e}");
                return None;
            }
        };

        let next = StatementSeq::from_tokens(tokens).and_then(|s| Some(Box::new(s)));

        Some(StatementSeq { stmt: ast, next })
    }
}

#[derive(Debug)]
pub struct Program {
    stmt: StatementSeq,
}

impl Program {
    pub fn new(mut tokens: Vec<Token>) -> Option<Self> {
        if let Some(Token::Reserved(t)) = tokens.pop() {
            if t != Reserved::End {
                return None;
            }
        } else {
            return None;
        }

        let mut tokens = tokens.into_iter().peekable();

        if let Some(Token::Reserved(t)) = tokens.next() {
            if t != Reserved::Begin {
                return None;
            }
        } else {
            return None;
        }

        Some(Self {
            stmt: StatementSeq::from_tokens(&mut tokens)?,
        })
    }
}

pub fn parse_ast(tokens: Vec<Token>) -> Program {
    Program::new(tokens).unwrap()
}
