#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::io::{self, Write};
use std::process::Command;

mod builtins;
use builtins::{run_cd, run_echo, run_exit, run_pwd, run_type};

// Función para dividir el input en comandos y argumentos
pub fn split_command(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut word = String::new();
    
    let mut in_quotes = false;
    let mut in_double_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\'' {
            in_quotes = !in_quotes;
        } else if c == '"' {
            in_double_quotes = !in_double_quotes;
        } else if c == ' ' && !in_quotes && !in_double_quotes {
            if !word.is_empty() {
                result.push(word.clone()); // Guardar palabra actual
                word.clear();
            }
        } else {
            word.push(c);
        }
    }

    if !word.is_empty() {
        result.push(word);
    }

    result
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Leer la entrada del usuario
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // Separar comando y argumentos
        let command_parts = split_command(input.trim());
        if command_parts.is_empty() {
            continue; // Si está vacío, pedir otro comando
        }

        let command = command_parts[0].clone(); // Clonar el primer elemento como comando
        let args: Vec<String> = command_parts.iter().skip(1).cloned().collect(); // Clonar argumentos

        match command.as_str() {
            "exit" => run_exit(args),
            "echo" => {
                let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                run_echo(args_str);
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
                    let exe_name = exe.to_str().unwrap();
                    let status = Command::new(exe_name).args(args).status().unwrap();
                    if !status.success() {
                        println!("{}: command failed with status {}", command, status);
                    }
                    continue;
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}
