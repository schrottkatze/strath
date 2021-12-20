use std::env;
use std::fs;

pub mod shell;
pub mod parsing;

use parsing::preprocessor::preprocess;

fn main() {
    let args: Vec<String> = env::args().collect();

    let run_shell = String::from("RUN_SH");
    let path: &String = match args.get(1) {
        Some(path) => path,
        None => &run_shell,
    };

    if path == "RUN_SH" {
        shell::run_shell();
    } else {
        let file = fs::read_to_string(path).expect("Couldnt read file:");

        let code = preprocess(file);
    }
}

