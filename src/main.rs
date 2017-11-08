use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

fn builtin_cd (args: &mut Iterator<Item=&str>) -> i32 {
    if let Some(path) = args.nth(0) {
        match env::set_current_dir(Path::new(path)) {
            Ok(()) => 0,
            Err(err) => {
                println!("{}", err.to_string());
                1
            }
        }
    } else {
        println!("Specify path.");
        1
    }
}

fn builtin_ls (args: &mut Iterator<Item=&str>) -> i32 {
    let path = match args.nth(0) {
        Some(p) => p,
        None => "./"
    };
    match fs::read_dir(path) {
        Ok(paths) => {
            for path in paths {
                println!("{}", path.unwrap().file_name().to_str().unwrap());
            }
            0
        },
        Err(err) => {
            println!("{}", err.to_string());
            1
        }
    }
}

fn read_eval_print() {
    // Show prompt
    print!("$ ");
    io::stdout().flush().ok();

    // Read line
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    // Evaluate
    let exit_commands = vec!["exit", "logout", "bye"];
    // Builtin functions
    // TODO: WTF this dirty type conversion...
    let builtin_commands: HashMap<_, _> = [
        ("cd", &builtin_cd as &Fn(&mut Iterator<Item=&str>) -> i32),
        ("ls", &builtin_ls),
    ].iter().cloned().collect();
    let mut args = input.split_whitespace();
    // Do nothing if an empty line is given
    if let Some(command) = args.nth(0) {
        // Exit
        if exit_commands.contains(&command) {
            println!("logout");
            exit(0);
        }
        // Execute builtin function
        if let Some(builtin_function) = builtin_commands.get(command) {
            builtin_function(&mut args);
        } else {
            // Echo command and arguments
            println!("{}", command);
            for arg in args {
                println!("{}", arg)
            }
        }
    }
}

fn main() {
    // REPL
    loop {
        read_eval_print();
    }
}
