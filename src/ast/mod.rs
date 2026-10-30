use super::token::*;
use anyhow::{anyhow, Result};
use std::iter::Peekable;

pub mod production;

use production::AstObj;

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
            _ => AstNode::Unknown,
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
    left: AstNode,
    op: Option<Operator>,
    right: Option<Box<Self>>,
}

impl Expr {
    pub fn new(objs: Vec<AstObj>) -> Option<Self> {
        let mut objs = objs.into_iter();

        if let Some(AstObj::AstNode(node)) = objs.next() {
            if let Some(AstObj::AstNode(AstNode::Operator(op))) = objs.next() {
                if op.op != Op::Equal {
                    if let Some(AstObj::Ast(Ast::Expr(expr))) = objs.next() {
                        return Some(Self {
                            left: node,
                            op: Some(op),
                            right: Some(Box::new(expr)),
                        });
                    }
                } else {
                    todo!("Unhandled grammar error this shouldnt be possible though")
                }
            } else {
                return Some(Self {
                    left: node,
                    op: None,
                    right: None,
                });
            }
        }

        None
    }
    pub fn from_nodes(mut nodes: impl Iterator<Item = AstNode>) -> Option<Self> {
        None
    }
}

#[derive(Debug)]
pub enum Ast {
    Procedure(Procedure),
    SS(StatementSeq),
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
    left: Var,
    right: Expr,
}

impl Assign {
    pub fn parse_ast<'a, I: Iterator<Item = Token<'a>>>(tokens: &mut Peekable<I>) -> Option<Self> {
        //match tokens.peek() {}
        None
    }
    pub fn new(objs: Vec<AstObj>) -> Option<Self> {
        let mut objs = objs.into_iter();
        if let Some(AstObj::AstNode(AstNode::Var(var))) = objs.next() {
            if let Some(AstObj::AstNode(AstNode::Operator(op))) = objs.next() {
                if op.op == Op::Equal {
                    if let Some(AstObj::Ast(Ast::Expr(expr))) = objs.next() {
                        return Some(Self {
                            left: var,
                            right: expr,
                        });
                    }
                }
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct StatementSeq {
    stmt: Box<Ast>,
    next: Option<Box<Self>>,
}

impl StatementSeq {
    pub fn new(asts: Vec<AstObj>) -> Self {
        let mut asts = asts.into_iter();
        let ast = asts.next();

        if let Some(AstObj::Ast(ast)) = ast {
            let next = if let Some(AstObj::Ast(Ast::SS(ss))) = asts.next() {
                Some(Box::new(ss))
            } else {
                None
            };

            Self {
                stmt: Box::new(ast),
                next,
            }
        } else {
            todo!("Not handled in ss yet");
        }
    }
}

#[derive(Debug)]
pub struct Procedure {
    stmt: StatementSeq,
}

impl Procedure {
    pub fn new(stmt: StatementSeq) -> Self {
        Self { stmt }
    }
}
