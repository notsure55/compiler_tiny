use super::grammar::*;
use super::token::{Token, TokenId};

use std::collections::VecDeque;

fn tokenize<'a>(
    input: &mut &'a str,
    index: usize,
    grammars: Vec<&dyn Grammar>,
) -> Option<Token<'a>> {
    if grammars.len() == 1 {
        let grammar = grammars.get(0)?;

        if !grammar.satisfied(input.get(..index)?) {
            let token = grammar.tokenize(input.get(..index - 1)?);
            *input = input.get(index - 1..)?;

            return Some(token);
        }

        return tokenize(input, index + 1, grammars);
    }

    let satisfied_grammars: Vec<_> = grammars
        .into_iter()
        .filter(|g| g.satisfied(input.get(..index).unwrap()))
        .collect();

    if satisfied_grammars.len() > 0 {
        tokenize(input, index + 1, satisfied_grammars)
    } else {
        None
    }
}

pub struct Lexer<'a> {
    tokens: VecDeque<Token<'a>>,
}

// TODO: Add a way to log line and columns for each token
// IDEAS: split into line off initial parsing, and configure a loop
// that keeps track of index in a certain line and stores that with the token
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, grammars: Vec<&'static dyn Grammar>) -> Self {
        let mut input = input;
        let mut tokens = VecDeque::new();
        while let Some(token) = tokenize(&mut input, 1, grammars.clone()) {
            if token.id() != TokenId::WhiteSpace {
                tokens.push_back(token);
            }
        }

        Self { tokens }
    }
    pub fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.front()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.pop_front()
    }
}
