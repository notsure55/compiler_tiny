use super::grammar::*;
use super::token::Token;

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
    input: &'a str,
    grammars: Vec<&'static dyn Grammar>,
}

// TODO: Add a way to log line and columns for each token
// IDEAS: split into line off initial parsing, and configure a loop
// that keeps track of index in a certain line and stores that with the token
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, grammars: Vec<&'static dyn Grammar>) -> Self {
        Self { input, grammars }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = tokenize(&mut self.input, 1, self.grammars.clone())?;

        if let Some(reserved) = token.reserved() {
            return Some(reserved);
        }

        Some(token)
    }
}
