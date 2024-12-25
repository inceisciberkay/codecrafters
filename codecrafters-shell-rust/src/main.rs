use std::io::Write;

mod cmd_parser;
use cmd_parser::{parse_quotes, parse_redirection};

mod cmd_handler;
use cmd_handler::{check_cmd_in_path, handle_path_cmd, CMD_MAP};

fn run(cmd: &str) {
    let args = parse_quotes(cmd);
    let (args, out_handle, err_handle) = parse_redirection(&args);

    let cmd = args[0];
    let args = &args[1..];

    if let Some(handle_builtin_cmd) = CMD_MAP.get(cmd.as_str()) {
        handle_builtin_cmd(args, out_handle, err_handle);
    } else if let Some(_) = check_cmd_in_path(cmd) {
        handle_path_cmd(cmd, args, out_handle, err_handle);
    } else {
        eprintln!("{}: command not found", cmd);
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();

    loop {
        // print prompt
        print!("$ ");
        std::io::stdout().flush().unwrap();

        // read input
        stdin.read_line(&mut input).unwrap();

        // run cmd
        let cmd = input.trim();
        run(cmd);

        input.clear();
    }
}
