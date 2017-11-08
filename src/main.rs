use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, exit};

fn builtin_cd (args: &mut Iterator<Item=&str>) -> i32 {
    if let Some(path) = args.nth(0) {
        match env::set_current_dir(Path::new(path)) {
            Ok(()) => 0,
            Err(err) => {
                eprintln!("{}", err.to_string());
                1
            }
        }
    } else {
        eprintln!("Specify path.");
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
            eprintln!("{}", err.to_string());
            1
        }
    }
}

fn do_command(command: &str, mut args: &mut Iterator<Item=&str>) -> i32 {
    // List of exit commands
    let exit_commands = vec!["exit", "logout", "bye"];
    // Builtin functions
    // TODO: WTF this dirty type conversion...
    let builtin_commands: HashMap<_, _> = [
        ("cd", &builtin_cd as &Fn(&mut Iterator<Item=&str>) -> i32),
        ("ls", &builtin_ls),
    ].iter().cloned().collect();

    // Exit
    if exit_commands.contains(&command) {
        eprintln!("logout");
        exit(0);
    }
    // Execute builtin function
    if let Some(builtin_function) = builtin_commands.get(command) {
        builtin_function(&mut args)
    } else {
        let child = Command::new(command)
                            .args(args)
                            .spawn();
        match child {
            Ok(mut child) => child.wait().unwrap().code().unwrap(),
            Err(err) => {
                eprintln!("{}", err.to_string());
                127
            }
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
    // Parse line and split
    let mut args = input.split_whitespace();
    // Do nothing if an empty line is given
    if let Some(command) = args.nth(0) {
        do_command(command, &mut args);
    }
}

fn main() {
    // REPL
    loop {
        read_eval_print();
    }
}
