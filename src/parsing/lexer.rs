use preprocessor::preprocess;

struct Lexer {
    code: String,
    pos: usize,
    current: Option<char>,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            code,
            pos: 1,
            current: None,
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
        self.code = preprocess(self.code);
        
        loop {
            //let token = match self.current {
                //Some('+') => Token::ADD,
                //Some('-') => Token::SUBTRACT,
                //Some('*') => Token::MULTIPLY,
                //Some('/') => Token::DIVIDE,
                //Some('(') => Token::LBRACK,
                //Some(')') => Token::RBRACK,
                //Some(' ') => continue,
                //Some('\n') => continue,
                //None => break,
                //Some(_) => continue,
            //};

            tokens.push(match self.current {
                Some('+') => Token::ADD,
                Some('-') => Token::SUBTRACT,
                Some('*') => Token::MULTIPLY,
                Some('/') => Token::DIVIDE,
                Some('(') => Token::LBRACK,
                Some(')') => Token::RBRACK,
                Some(' ') => continue,
                Some('\n') => continue,
                None => break,
                Some(_) => continue,
            });
            self.next();
        }

        println!("{:?}", tokens);
    }
}
