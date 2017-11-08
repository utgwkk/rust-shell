use std::io::{self, Write};
use std::process::exit;

fn read_eval_print() {
    // Show prompt
    print!("$ ");
    io::stdout().flush().ok();

    // Read line
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    // Evaluate
    let exit_commands = vec!["exit", "logout", "bye"];
    let mut args = input.split_whitespace();
    // Do nothing if an empty line is given
    if let Some(command) = args.nth(0) {
        // Exit
        if exit_commands.contains(&command) {
            println!("logout");
            exit(0);
        }
        // Echo command and arguments
        println!("{}", command);
        for arg in args {
            println!("{}", arg)
        }
    }
}

fn main() {
    // REPL
    loop {
        read_eval_print();
    }
}
