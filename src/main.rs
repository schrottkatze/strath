use std::env;
use std::fs;

mod preprocessor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("Couldnt read file:");

    println!("{:?}", preprocessor::preprocess(file));
}

fn parse_and_interpret(code: String) {

}

