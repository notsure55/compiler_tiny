use super::{Assign, Ast, AstNode, Expr, Token, TokenId};
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
pub struct ExprBuilder {}

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

// Will be called if you know it is this the first token is part of this ast
impl ExprBuilder {
    pub fn build<'a, I: Iterator<Item = Token<'a>>>(
        &self,
        tokens: &mut Peekable<I>,
    ) -> Result<Vec<AstObj>> {
        let mut ast_rules = vec![];

        let token = tokens.next().unwrap();

        let prod = self.prod();

        let rule = prod.find_match(&token, tokens).unwrap();
        println!("{rule:?}");

        ast_rules.push(AstObj::from_token(token));

        for term in rule.iter() {
            let ast_rule = match term {
                Terminal::Term(builder) => ast_rules.append(&mut builder.build(tokens)?),
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
    pub fn prod(&self) -> ProductionRule {
        ProductionRule::new(vec![
            vec![
                Terminal::Term(ExprBuilder {}),
                Terminal::Non(TokenId::Plus),
                Terminal::Term(ExprBuilder {}),
            ],
            vec![
                Terminal::Term(ExprBuilder {}),
                Terminal::Non(TokenId::Plus),
                Terminal::Term(ExprBuilder {}),
            ],
            vec![Terminal::Non(TokenId::IntLit)],
            vec![Terminal::Non(TokenId::Ident)],
        ])
    }
    pub fn is_first_variant<'a>(&self, token: &Token<'a>) -> bool {
        for sub_rule in self.prod().rules.iter() {
            if let Some(term) = sub_rule.first() {
                match term {
                    // TODO implement somethign to check if self so if self we skip
                    Terminal::Term(builder) => continue, //return builder.is_first_variant(token),
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
}

/*pub trait AstBuilder {
    fn build<'a, I: Iterator<Item = Token<'a>>>(&self, tokens: &mut Peekable<I>)
        -> Result<AstObj>;
    fn production_rules(&self) -> &ProductionRule;
}*/

#[derive(Debug)]
pub enum Terminal {
    Term(ExprBuilder),
    Non(TokenId),
}

#[derive(Debug)]
pub struct ProductionRule {
    rules: Vec<Vec<Terminal>>,
}

impl ProductionRule {
    pub fn new(rules: Vec<Vec<Terminal>>) -> Self {
        Self { rules }
    }

    pub fn find_match<'a, I: Iterator<Item = Token<'a>>>(
        &self,
        token: &Token<'a>,
        tokens: &mut Peekable<I>,
    ) -> Option<&[Terminal]> {
        for sub_rule in self.rules.iter() {
            let first = sub_rule.first().unwrap();
            match first {
                Terminal::Term(ast) => {
                    if ast.is_first_variant(token) {
                        if let Some(term) = sub_rule.get(1) {
                            if let Some(peeked) = tokens.peek() {
                                match term {
                                    Terminal::Term(ast) => {
                                        if ast.is_first_variant(peeked) {
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
