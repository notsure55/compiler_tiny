use super::token::{Op as Operator, Token};
use regex::Regex;

const RESERVED_TOKENS: [&'static str; 2] = ["BEGIN", "END"];
pub const REGEX_IDENT: &'static str = r"[a-zA-Z_]{1}\w*";
pub const REGEX_WHITESPACE: &'static str = r"\s";
pub const REGEX_OPERATOR: &'static str = r"(=|-|\+)";
pub const REGEX_INT_LIT: &'static str = r"\d+";
pub const REGEX_SEMICOLON: &'static str = r";";

pub trait Grammar {
    fn regex(&self) -> &Regex;
    fn satisfied(&self, input: &str) -> bool {
        if let Some(found) = self.regex().find_at(input, 0) {
            let range = found.range();

            if range.start == 0 && range.end == input.len() {
                return true;
            }
        }

        false
    }
    fn tokenize<'a>(&self, input: &'a str) -> Token<'a>;
}

macro_rules! grammar_rule {
    ($name: ident, $tokenizer: expr, $global_name: ident,  $regex: literal) => {
        pub static $global_name: std::sync::LazyLock<$name> =
            std::sync::LazyLock::new(|| $name::new($regex));

        pub struct $name {
            regex: Regex,
        }

        impl $name {
            pub fn new(regex: &str) -> Self {
                Self {
                    regex: Regex::new(regex).unwrap(),
                }
            }
        }

        impl Grammar for $name {
            fn regex(&self) -> &Regex {
                &self.regex
            }
            fn tokenize<'a>(&self, input: &'a str) -> Token<'a> {
                $tokenizer(input)
            }
        }
    };
}

grammar_rule!(
    IdentGrammar,
    |input: &'a str| -> Token<'a> { Token::Ident(input) },
    IDENT_GRAMMAR,
    r"[a-zA-Z_]{1}\w*"
);

grammar_rule!(
    WhiteSpaceGrammar,
    |input: &'a str| -> Token<'a> { Token::WhiteSpace(input) },
    WHITESPACE_GRAMMAR,
    r"\s"
);

grammar_rule!(
    OperatorGrammar,
    |input: &'a str| -> Token<'a> {
        let op = match input {
            "+" => Operator::Plus,
            "=" => Operator::Equal,
            "-" => Operator::Minus,
            _ => todo!("Hanlde this maybe? shouldnt be possible doe?"),
        };

        Token::Operator(op)
    },
    OPERATOR_GRAMMAR,
    r"(=|-|\+)"
);

grammar_rule!(
    IntLitGrammar,
    |input: &'a str| -> Token<'a> { Token::IntLit(i32::from_str_radix(input, 10).unwrap()) },
    INT_LIT_GRAMMAR,
    r"\d+"
);

grammar_rule!(
    SemiColonGrammar,
    |input: &'a str| -> Token<'a> { Token::SemiColon },
    SEMI_COLON_GRAMMAR,
    r";"
);
