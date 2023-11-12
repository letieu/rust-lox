mod error;
mod token_type;
mod token;
mod scanner;

use std::{env::args, process, fs, io::{self, BufRead}};

use scanner::Scanner;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    }

    if args.len() == 2 {
        run_file(&args[1]).expect("Error running file");
        return;
    }

    run_prompt();
}

fn run_file(path: &str) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(path)?;
    run(contents);
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();

    print!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
        } else {
            break;
        }
    }
}

fn run(contents: String) {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
