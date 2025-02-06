use pathsearch::find_executable_in_path;
static BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

pub fn run_exit(args: Vec<String>) {
    let code = args.get(0).map_or(0, |c| c.parse().unwrap_or(0));
    std::process::exit(code);
}

pub fn run_echo(args: Vec<String>) {
    println!("{}", args.join(" "));
}

pub fn run_type(args: Vec<String>) {
    let arg = args.get(0).unwrap();
    if BUILTINS.contains(&arg.as_str()) {
        println!("{} is a shell builtin", arg);
    } else if let Some(exe) = find_executable_in_path(arg) {
        println!("{} is {}", arg, exe.display());
    } else {
        println!("{} not found", arg);
    }
}

pub fn run_pwd() {
    println!("{}", std::env::current_dir().unwrap().display());
}

pub fn run_cd(args: Vec<String>) {
    let default_path = "/".to_string();
    let path = args.get(0).unwrap_or(&default_path);
    if path == &"~" {
        std::env::set_current_dir(std::env::var("HOME").unwrap()).unwrap();
    } else if let Err(_e) = std::env::set_current_dir(path) {
        println!("cd: {}: No such file or directory", path);
    }
}
