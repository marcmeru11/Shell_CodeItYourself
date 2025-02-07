#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::io::{self, Write};
use std::process::Command;

mod builtins;
use builtins::{run_cd, run_echo, run_exit, run_pwd, run_type};

mod utils;
use utils::split_command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command_parts = split_command(input.trim());
        if command_parts.is_empty() {
            continue;
        }

        let command = command_parts[0].clone();
        let args: Vec<String> = command_parts.iter().skip(1).cloned().collect();

        match command.as_str() {
            "exit" => run_exit(args),
            "echo" => {
                run_echo(args);
                continue;
            }
            "type" => {
                run_type(args);
                continue;
            }
            "pwd" => {
                run_pwd();
                continue;
            }
            "cd" => {
                run_cd(args);
                continue;
            }
            _ => {
                if let Some(exe) = find_executable_in_path(&command) {
                    if let Some(exe_name) = exe.file_name().and_then(|name| name.to_str()) {
                        let status = Command::new(exe_name).args(args).status().unwrap();
                        if !status.success() {
                            println!("{}: command failed with status {}", command, status);
                        }
                        continue;
                    }
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
        print!("{}", input);
    }
}
