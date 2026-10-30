use super::{Assign, Ast, AstNode, Expr, Token, TokenId};
use crate::lexer::Lexer;
use std::fmt::Debug;
use std::iter::Peekable;

use anyhow::{anyhow, Result};

/*
y = 10 - x;
S -> Term-Assign;
Assign -> NonTerminal-Ident NonTerminal-Equal Term-Expr;
Expr-> (Term-Expr NonTerm-Plus Term-Expr)|(Term-Expr NonTerm-Minus TermExpr)|NonTerm-Constant|NonTerm-Ident
 */

/*macro_rules! production_rule {
    ($($term: expr) +) => {};
}*/

#[derive(Debug)]
pub enum AstObj {
    Ast(Ast),
    AstNode(AstNode),
}

impl AstObj {
    pub fn from_token<'a>(token: Token<'a>) -> Self {
        Self::AstNode(AstNode::new(token).unwrap())
    }
    pub fn from_tokens<'a>(tokens: &[Token<'a>]) -> Option<Self> {
        None
    }
}

use std::sync::LazyLock;

// Expr-> (Term-Expr NonTerm-Plus Term-Expr)|(Term-Expr NonTerm-Minus TermExpr)|NonTerm-Constant|NonTerm-Ident
static EXPR_RULES: LazyLock<ProductionRule> = LazyLock::new(|| {
    ProductionRule::new(vec![
        vec![
            Terminal::Term(Box::new(ExprBuilder {})),
            Terminal::Non(TokenId::Plus),
            Terminal::Term(Box::new(ExprBuilder {})),
        ],
        vec![
            Terminal::Term(Box::new(ExprBuilder {})),
            Terminal::Non(TokenId::Plus),
            Terminal::Term(Box::new(ExprBuilder {})),
        ],
        vec![Terminal::Non(TokenId::IntLit)],
        vec![Terminal::Non(TokenId::Ident)],
    ])
});

// Assign -> NonTerminal-Ident NonTerminal-Equal Term-Expr;
static ASSIGN_RULES: LazyLock<ProductionRule> = LazyLock::new(|| {
    ProductionRule::new(vec![vec![
        Terminal::Non(TokenId::Ident),
        Terminal::Non(TokenId::Equal),
        Terminal::Term(Box::new(ExprBuilder {})),
    ]])
});

#[derive(Debug)]
pub struct ExprBuilder {}

impl AstBuilder for ExprBuilder {
    fn prod(&self) -> &ProductionRule {
        &*EXPR_RULES
    }
}

#[derive(Debug)]
pub struct AssignBuilder {}

impl AstBuilder for AssignBuilder {
    fn prod(&self) -> &ProductionRule {
        &*ASSIGN_RULES
    }
}

pub trait AstBuilder: Send + Sync + Debug {
    fn build<'a>(&self, tokens: &mut Lexer<'a>) -> Result<Vec<AstObj>> {
        let mut ast_rules = vec![];

        let token = tokens.next().unwrap();

        let prod = self.prod();

        let rule = prod.find_match(&token, tokens).unwrap();
        println!("{rule:?}");

        ast_rules.push(AstObj::from_token(token));

        for term in rule.iter() {
            let ast_rule = match term {
                Terminal::Term(builder) => ast_rules.extend(builder.build(tokens)?),
                Terminal::Non(token_id) => {
                    if let Some(peeked) = tokens.peek() {
                        if peeked.id() == *token_id {
                            let token = tokens.next().unwrap();
                            ast_rules.push(AstObj::from_token(token))
                        } else {
                            return Err(anyhow!(
                                "Invalid syntax for rule: {rule:?} token: {peeked:?}"
                            ));
                        }
                    } else {
                        break;
                    }
                }
            };
        }

        println!("Curent rule being built! {rule:?}");

        Ok(ast_rules)
    }
    fn prod(&self) -> &ProductionRule;
    fn is_first_variant<'a>(&self, token: &Token<'a>) -> bool {
        for sub_rule in self.prod().rules.iter() {
            if let Some(term) = sub_rule.first() {
                match term {
                    Terminal::Term(builder) => {
                        if self.is_self(builder) {
                            continue;
                        } else {
                            return builder.is_first_variant(token);
                        }
                    }
                    Terminal::Non(token_id) => {
                        if token.id() == *token_id {
                            return true;
                        }
                    }
                }
            } else {
                continue;
            }
        }

        false
    }
    fn is_self(&self, other: &Box<dyn AstBuilder>) -> bool {
        if self.prod() == other.prod() {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub enum Terminal {
    Term(Box<dyn AstBuilder>),
    Non(TokenId),
}

unsafe impl Sync for Terminal {}

#[derive(Debug)]
pub struct ProductionRule {
    rules: Vec<Vec<Terminal>>,
}

unsafe impl Sync for ProductionRule {}

impl ProductionRule {
    pub fn new(rules: Vec<Vec<Terminal>>) -> Self {
        Self { rules }
    }

    pub fn find_match<'a>(&self, token: &Token<'a>, tokens: &mut Lexer<'a>) -> Option<&[Terminal]> {
        for sub_rule in self.rules.iter() {
            let first = sub_rule.first().unwrap();
            match first {
                Terminal::Term(ast) => {
                    if ast.is_first_variant(token) {
                        if let Some(term) = sub_rule.get(1) {
                            if let Some(peeked) = tokens.peek() {
                                match term {
                                    Terminal::Term(ast) => {
                                        if ast.is_first_variant(&peeked) {
                                            return Some(&sub_rule[1..]);
                                        } else {
                                            continue;
                                        }
                                    }
                                    Terminal::Non(token_id) => {
                                        if peeked.id() == *token_id {
                                            return Some(&sub_rule[1..]);
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        return Some(&sub_rule[1..]);
                    }
                }
                Terminal::Non(token_id) => {
                    if token.id() == *token_id {
                        return Some(&sub_rule[1..]);
                    } else {
                        continue;
                    }
                }
            }
        }

        None
    }
}

impl PartialEq for ProductionRule {
    fn eq(&self, other: &ProductionRule) -> bool {
        true
    }
}
