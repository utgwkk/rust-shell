use std::io::{self, Write};

fn read_eval_print() {
    // Show prompt
    print!("$ ");
    io::stdout().flush().ok();

    // Read line
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    // Echo
    print!("{}", input);
}

fn main() {
    // REPL
    loop {
        read_eval_print();
    }
}
