use super::{Assign, Ast, AstNode, Expr, Procedure, StatementSeq, Token, TokenId};
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
            Terminal::Non(TokenId::Minus),
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
        Terminal::Non(TokenId::SemiColon),
    ]])
});

// Procedure -> Terminal::Non(TokenId::Begin) Terminal::Term(SS) Terminal::Non(TokenId::End);
static PROCEDURE_RULES: LazyLock<ProductionRule> = LazyLock::new(|| {
    ProductionRule::new(vec![vec![
        Terminal::Non(TokenId::Begin),
        Terminal::Term(Box::new(SSBuilder {})),
        Terminal::Non(TokenId::End),
    ]])
});

// SS -> S Option<SS>
// S -> Assign
static SS_RULES: LazyLock<ProductionRule> = LazyLock::new(|| {
    ProductionRule::new(vec![vec![
        Terminal::Term(Box::new(AssignBuilder {})),
        Terminal::Term(Box::new(SSBuilder {})),
    ]])
});

#[derive(Debug)]
pub struct ExprBuilder {}

impl AstBuilder for ExprBuilder {
    fn prod(&self) -> &ProductionRule {
        &*EXPR_RULES
    }
    fn create_self(&self, asts: Vec<AstObj>) -> Ast {
        Ast::Expr(Expr::new(asts).unwrap())
    }
}

#[derive(Debug)]
pub struct AssignBuilder {}

impl AstBuilder for AssignBuilder {
    fn prod(&self) -> &ProductionRule {
        &*ASSIGN_RULES
    }
    fn create_self(&self, asts: Vec<AstObj>) -> Ast {
        Ast::Assign(Assign::new(asts).unwrap())
    }
}

#[derive(Debug)]
pub struct ProcedureBuilder {}

impl AstBuilder for ProcedureBuilder {
    fn prod(&self) -> &ProductionRule {
        &*PROCEDURE_RULES
    }
    fn create_self(&self, asts: Vec<AstObj>) -> Ast {
        let mut asts = asts.into_iter();
        let _ = asts.next(); // should be begin 3rd should be end from production rule;
        if let Some(AstObj::Ast(Ast::SS(ss))) = asts.next() {
            Ast::Procedure(Procedure::new(ss))
        } else {
            todo!("Unhandled case of procedure not actually being a procedure?");
        }
    }
}

#[derive(Debug)]
pub struct SSBuilder {}

impl AstBuilder for SSBuilder {
    fn prod(&self) -> &ProductionRule {
        &*SS_RULES
    }
    fn create_self(&self, asts: Vec<AstObj>) -> Ast {
        Ast::SS(StatementSeq::new(asts))
    }
}

pub trait AstBuilder: Send + Sync + Debug {
    fn build<'a>(&self, tokens: &mut Lexer<'a>) -> Option<Ast> {
        let mut ast_rules = vec![];

        let prod = self.prod();

        let (rule, token) = prod.find_match(tokens);

        if let Some(t) = token {
            ast_rules.push(AstObj::from_token(t));
        }

        if let Some(rule) = rule {
            println!("{rule:?}");

            for term in rule.iter() {
                let ast_rule = match term {
                    Terminal::Term(builder) => {
                        if let Some(ast) = builder.build(tokens) {
                            ast_rules.push(AstObj::Ast(ast))
                        } else {
                            break;
                        }
                    }
                    Terminal::Non(token_id) => {
                        if let Some(peeked) = tokens.peek() {
                            if peeked.id() == *token_id {
                                let token = tokens.next().unwrap();
                                ast_rules.push(AstObj::from_token(token))
                            } else {
                                todo!("Invalid syntax for rule: {rule:?} token: {peeked:?}");
                            }
                        } else {
                            break;
                        }
                    }
                };
            }

            Some(self.create_self(ast_rules))
        } else {
            None
        }
    }
    fn create_self(&self, asts: Vec<AstObj>) -> Ast;
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
    fn is_variant<'a>(&self, token: &Token<'a>) -> bool {
        for sub_rule in self.prod().rules.iter() {
            if sub_rule.len() == 1 {
                match &sub_rule[0] {
                    Terminal::Term(builder) => {
                        continue;
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

impl PartialEq for Terminal {
    fn eq(&self, other: &Terminal) -> bool {
        match other {
            Terminal::Term(boxed) => match self {
                Terminal::Term(boxed1) => Box::as_ptr(boxed).addr() == Box::as_ptr(boxed1).addr(),
                Terminal::Non(_) => false,
            },
            Terminal::Non(id) => match self {
                Terminal::Term(_) => false,
                Terminal::Non(id1) => id == id1,
            },
        }
    }
}

unsafe impl Sync for Terminal {}

#[derive(Debug, PartialEq)]
pub struct ProductionRule {
    rules: Vec<Vec<Terminal>>,
}

unsafe impl Sync for ProductionRule {}

impl ProductionRule {
    pub fn new(rules: Vec<Vec<Terminal>>) -> Self {
        Self { rules }
    }

    pub fn find_match<'a>(
        &self,
        tokens: &mut Lexer<'a>,
    ) -> (Option<&[Terminal]>, Option<Token<'a>>) {
        let token = tokens.peek().unwrap();

        for sub_rule in self.rules.iter() {
            let first = sub_rule.first().unwrap();

            match first {
                Terminal::Term(ast) => {
                    if ast.is_first_variant(token) {
                        if let Some(term) = sub_rule.get(1) {
                            if let Some(peeked) = tokens.peek_next() {
                                match term {
                                    Terminal::Term(ast) => {
                                        if ast.is_first_variant(&peeked) {
                                            continue;
                                        }
                                    }
                                    Terminal::Non(token_id) => {
                                        if peeked.id() == *token_id {
                                            return (Some(&sub_rule[1..]), tokens.next());
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                            }
                        }

                        return (Some(&sub_rule), None);
                    } else {
                        continue;
                    }
                }
                Terminal::Non(token_id) => {
                    if token.id() == *token_id {
                        return (Some(&sub_rule), None);
                    } else {
                        continue;
                    }
                }
            }
        }

        (None, None)
    }
}
