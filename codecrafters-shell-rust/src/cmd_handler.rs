use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command, Stdio};
use std::{env, fs};

type CmdHandler = fn(&[&String], Box<dyn Write>, Box<dyn Write>) -> ();
type CmdMap = HashMap<&'static str, CmdHandler>;

lazy_static! {
    pub static ref CMD_MAP: CmdMap = {
        let mut cmd_map = CmdMap::new();
        cmd_map.insert("exit", handle_exit);
        cmd_map.insert("echo", handle_echo);
        cmd_map.insert("type", handle_type);
        cmd_map.insert("pwd", handle_pwd);
        cmd_map.insert("cd", handle_cd);
        cmd_map
    };
}

pub fn check_cmd_in_path(cmd: &str) -> Option<PathBuf> {
    if let Ok(paths) = env::var("PATH") {
        for entry in paths
            .split(':')
            .map(|path| fs::read_dir(path))
            .filter_map(Result::ok)
            .flatten()
            .flat_map(Result::ok)
        {
            if cmd == entry.file_name().to_string_lossy() {
                return Some(entry.path());
            }
        }
    }
    None
}

pub fn handle_path_cmd(
    cmd: &str,
    args: &[&String],
    mut out_handle: Box<dyn Write>,
    mut err_handle: Box<dyn Write>,
) {
    let output = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    out_handle.write_all(&output.stdout).unwrap();
    err_handle.write_all(&output.stderr).unwrap();
}

fn handle_exit(args: &[&String], mut _out_handle: Box<dyn Write>, mut err_handle: Box<dyn Write>) {
    if args.len() != 1 {
        writeln!(err_handle, "exit: invalid number of args").unwrap();
        return;
    }

    if let Ok(exit_code) = args[0].parse::<i32>() {
        exit(exit_code);
    } else {
        writeln!(err_handle, "exit: invalid argument: {}", &args[0]).unwrap();
    }
}

fn handle_echo(args: &[&String], mut out_handle: Box<dyn Write>, mut _err_handle: Box<dyn Write>) {
    let output: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    writeln!(out_handle, "{}", output.join(" ")).unwrap();
}

fn handle_type(args: &[&String], mut out_handle: Box<dyn Write>, mut err_handle: Box<dyn Write>) {
    if args.len() != 1 {
        writeln!(err_handle, "type: invalid number of args").unwrap();
        return;
    }

    let cmd = &args[0];
    if CMD_MAP.contains_key(cmd.as_str()) {
        writeln!(out_handle, "{} is a shell builtin", cmd).unwrap();
    } else {
        if let Some(path) = check_cmd_in_path(cmd) {
            writeln!(out_handle, "{} is {}", cmd, &path.to_string_lossy()).unwrap();
        } else {
            writeln!(err_handle, "{}: not found", cmd).unwrap();
        }
    }
}

fn handle_pwd(args: &[&String], mut out_handle: Box<dyn Write>, mut err_handle: Box<dyn Write>) {
    if args.len() != 0 {
        writeln!(err_handle, "pwd: invalid number of args").unwrap();
        return;
    }

    match env::current_dir() {
        Ok(path) => writeln!(out_handle, "{}", path.display()).unwrap(),
        Err(e) => writeln!(
            err_handle,
            "Current working directory cannot be accessed: {}",
            e
        )
        .unwrap(),
    };
}

fn handle_cd(args: &[&String], mut _out_handle: Box<dyn Write>, mut err_handle: Box<dyn Write>) {
    if args.len() != 1 {
        writeln!(err_handle, "cd: invalid number of args").unwrap();
        return;
    }

    let cd_path = if args[0] == "~" {
        &env::var("HOME").unwrap()
    } else {
        &args[0]
    };

    if let Err(_) = env::set_current_dir(cd_path) {
        writeln!(err_handle, "cd: {}: No such file or directory", &args[0]).unwrap();
    }
}
