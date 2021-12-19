use std::io;

pub fn run_shell() {
    'shell: loop {
        let string = get_line();
        if string.eq("q\n") || string.eq("quit\n") {
            break 'shell;
        }
        println!("{}", string);
    }
}

fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

