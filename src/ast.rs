use super::token::*;
use anyhow::{anyhow, Result};

#[derive(Debug)]
enum AstNode {
    Expr(Expr),
    Var(Var),
    Constant(Constant),
    Operator(Operator),
}

impl AstNode {
    pub fn new<'a>(tokens: Vec<Token<'a>>) -> Option<Self> {
        let mut nodes = vec![];
        for token in tokens.into_iter() {
            let node = match token {
                Token::IntLit(i) => AstNode::Constant(Constant::new(i)),
                Token::Ident(i) => AstNode::Var(Var::new(i)),
                Token::Operator(o) => AstNode::Operator(Operator::new(o)),
                _ => todo!("Handle this case!"),
            };

            nodes.push(node)
        }

        if nodes.len() > 1 {
            Expr::from_nodes(nodes.into_iter()).and_then(|e| Some(AstNode::Expr(e)))
        } else {
            nodes.into_iter().next()
        }
    }
}

// TODO: can add a type for the info, like float, int or whatevs
// TODO: operator precedence when adding new ops like *
#[derive(Debug)]
struct Operator {
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
struct Constant {
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
struct Var {
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
struct Info {
    line: u32,
    col: u32,
}

#[derive(Debug)]
struct Expr {
    left: Box<AstNode>,
    op: Option<Operator>,
    right: Option<Box<AstNode>>,
}

impl Expr {
    pub fn new(tokens: &[Token]) -> Option<Self> {
        None
    }
    pub fn from_nodes(mut nodes: impl Iterator<Item = AstNode>) -> Option<Self> {
        let left = Box::new(nodes.next()?);
        let op = if let Some(op) = nodes.next() {
            if let AstNode::Operator(op) = op {
                Some(op)
            } else {
                todo!("expr some how got a non operator as its second nodes? {op:?}");
            }
        } else {
            None
        };

        let right = Self::from_nodes(nodes).and_then(|e| Some(Box::new(AstNode::Expr(e))));

        Some(Self { left, op, right })
    }
}

#[derive(Debug)]
enum Ast {
    Assign(Assign),
}

impl Ast {
    pub fn nodes(&self) -> Vec<&AstNode> {
        match self {
            Self::Assign(a) => {
                vec![&a.left, &a.right]
            }
        }
    }
    pub fn new<'a>(tokens: &mut impl Iterator<Item = Token<'a>>) -> Result<Self> {
        let ast = Self::parse_ast(tokens)?;
        Ok(ast)
    }
    fn parse_ast<'a>(tokens: &mut impl Iterator<Item = Token<'a>>) -> Result<Self> {
        Ok(Ast::Assign(Assign::parse_ast(tokens)?))
    }
}

/*macro_rules! ast_grammar {
    ($ast: ty => $($node:ty) +) => {
        pub fn grammar(tokens: &[Token]) -> Option<$ast> {}
    };
}*/

//ast_grammar!(Assign => Ident Equal Expr SemiColon);
//ast_grammar!(Expr => Ident Equal Expr);

#[derive(Debug)]
struct Assign {
    left: AstNode,
    right: AstNode,
}

impl Assign {
    pub fn parse_ast<'a>(tokens: &mut impl Iterator<Item = Token<'a>>) -> Result<Self> {
        if let Some(Token::Ident(id)) = tokens.next() {
            if let Some(Token::Operator(op)) = tokens.next() {
                if op == Op::Equal {
                    //let (expr, next_tokens) ={
                    let mut expr = vec![];

                    for token in tokens {
                        if let Token::SemiColon = token {
                            break;
                        } else {
                            expr.push(token);
                        }
                    }

                    let node = AstNode::new(expr).unwrap();

                    return Ok(Self {
                        left: AstNode::Var(Var::new(id)),
                        right: node,
                    });
                } else {
                    return Err(anyhow!(
                        "Operator for assignment not an equal sign at [INSERT LINE AND COLON]"
                    ));
                }
            } else {
                return Err(anyhow!(
                    "Random ident with no operator as next token? [INSERT LINE AND COLON]"
                ));
            }
        }

        Err(anyhow!(
            "Inital token for assignment not a identifier [INSERT LINE AND COLON]"
        ))
    }
}

#[derive(Debug)]
struct StatementSeq {
    stmt: Ast,
    next: Option<Box<Self>>,
}

impl StatementSeq {
    pub fn new(stmt: Ast, next: Option<Box<Self>>) -> Self {
        Self { stmt, next }
    }
    pub fn from_tokens<'a>(tokens: &mut impl Iterator<Item = Token<'a>>) -> Option<Self> {
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

        let mut tokens = tokens.into_iter();

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
