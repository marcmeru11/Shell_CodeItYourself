use pathsearch::find_executable_in_path;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

const BUILTINS: [&str; 3] = ["exit", "echo", "type"];

fn run_exit(args: Vec<&str>) {
    let code = args.get(0).map_or(0, |c| c.parse().unwrap_or(0));
    std::process::exit(code);
}

fn run_echo(args: Vec<&str>) {
    println!("{}", args.join(" "));
}

fn run_type(args: Vec<&str>) {
    let arg = args.get(0).unwrap();
    if BUILTINS.contains(arg) {
        println!("{} is a shell builtin", arg);
    } else if let Some(exe) = find_executable_in_path(arg) {
        println!("{} is {}",arg, exe.display());
    } else {
        println!("{} not found", arg);
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command = input.trim().split_whitespace();
        let args = command.clone().skip(1);
        match command.clone().next() {
            Some("exit") => run_exit(args.collect()),
            Some("echo") => {
                run_echo(args.collect());
                continue;
            }
            Some("type") => {
                run_type(args.collect());
                continue;
            }
            _ => {
                if let Some(exe) = find_executable_in_path(command.clone().next().unwrap()) {
                    let exe_name = exe.file_name().unwrap().to_str().unwrap();
                    let status = Command::new(exe_name).args(args).status().unwrap();
                    if !status.success() {
                        println!("{}: command failed with status {}", command.clone().next().unwrap(), status);
                    }
                    continue;
                }
            }
        }
        println!("{}: command not found", input.trim());
    }
}
