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
        let mut args = command.clone().skip(1);

        match command.clone().next() {
            Some("exit") => exit(args.next().unwrap_or("0").parse().unwrap()),
            Some("echo") => {
                println!("{}", args.collect::<Vec<&str>>().join(" "));
                continue;
            }
            Some("type") => {
                let command = args.next().unwrap();
                match command {
                    "exit" => println!("exit is a shell builtin"),
                    "echo" => println!("echo is a shell builtin"),
                    "type" => println!("type is a shell builtin"),
                    _ => println!("{}: not found", command),
                }
                continue;
            }
            _ => {}
            
        }

        println!("{}: command not found", input.trim())
    }
}
