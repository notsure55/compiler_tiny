#![allow(unused)]

use anyhow::{anyhow, Result};
use regex::Regex;
use std::cell::LazyCell;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::iter::Peekable;
use std::str::Chars;

mod ast;
mod grammar;
mod lexer;
mod token;

use grammar::*;
use lexer::Lexer;
use token::Token;

/*
Expr -> (ident|Expr|const) (+|-) (ident|Expr|const)
Stmt -> ident equal Expr
Procedure -> reserved::begin whitespace (Stmt | Expr)* reserved::end whitespace
Program -> Procedure+
*/

// Program
// Procedure
// Stmt
// [ident y] [equal] [6 intlit] ->
// [ident x] [equal] [5 intlit] [plus] [ident y]

fn create_ast() {}

fn main() -> Result<()> {
    let mut buffer = String::new();
    File::open("../test.tiny")?.read_to_string(&mut buffer)?;

    let tokens: Vec<_> = Lexer::new(
        &buffer,
        vec![
            &*IDENT_GRAMMAR,
            &*WHITESPACE_GRAMMAR,
            &*OPERATOR_GRAMMAR,
            &*INT_LIT_GRAMMAR,
            &*SEMI_COLON_GRAMMAR,
        ],
    )
    .filter_map(|t| {
        if let Token::WhiteSpace(t) = t {
            None
        } else {
            Some(t)
        }
    })
    .collect();

    let program = ast::parse_ast(tokens);

    println!("{program:#?}");

    Ok(())
}
