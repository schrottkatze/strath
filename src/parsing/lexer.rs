use super::preprocessor::preprocess;
use super::token::Token;

pub struct Lexer {
    code: String,
    pos: usize,
    current: Option<char>,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            code: code.clone(),
            pos: 0,
            current: code.chars().nth(0),
        }
    }

    pub fn next(&mut self) {
        self.pos += 1;
        self.current = if self.pos < self.code.len() {
            self.code.chars().nth(self.pos)
        } else {
            None
        };
    }

    pub fn tokenize(&mut self) {
        let mut tokens: Vec<Token> = vec![];
        self.code = preprocess(self.code.clone());
        
        loop {
            let token: Option<Token> = match self.current {
                Some('+') => Some(Token::ADD),
                Some('-') => Some(Token::SUBTRACT),
                Some('*') => Some(Token::MULTIPLY),
                Some('/') => Some(Token::DIVIDE),
                Some('(') => Some(Token::LBRACK),
                Some(')') => Some(Token::RBRACK),
                Some(' ') => None,
                Some('\n') => None,
                None => break,
                Some(_) => None,
            };

            match token {
                Some(token) => {
                    tokens.push(token);
                    self.next();
                },
                None => self.next(),
            }
        }

        println!("{:?}", tokens);
    }
}
