use std::io;

use super::parsing::lexer::Lexer;

pub fn run_shell() {
    'shell: loop {
        let string = get_line();
        if string.eq("q\n") || string.eq("quit\n") {
            break 'shell;
        }
        let mut lexer = Lexer::new(string);
        lexer.tokenize();
        //println!("{}", string);
    }
}

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

