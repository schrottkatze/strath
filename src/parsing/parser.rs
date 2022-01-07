use super::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens
        }
    }

    fn analyze_math(&self, tokens: Vec<Token>) {
    }
}
