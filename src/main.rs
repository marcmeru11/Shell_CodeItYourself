#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command = input.trim().split_whitespace();

        match command.clone().next() {
            Some("exit") => exit(input.trim().split_whitespace().nth(1).unwrap_or("0").parse().unwrap()),
            /*
            Some("echo") => {
                let args = command.skip(1);
                println!("{}", args.collect::<Vec<&str>>().join(" "));
                continue;
            }
            */
            _ => {}
            
        }

        println!("{}: command not found", input.trim())
    }
}
