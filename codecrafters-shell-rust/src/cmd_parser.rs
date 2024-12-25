use std::fs;
use std::io::{BufWriter, Write};

pub fn parse_quotes(args_str: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut arg = String::new();
    let mut inside_single_quotes = false;
    let mut inside_double_quotes = false;
    let mut backslash = false;

    for c in args_str.chars() {
        if !inside_single_quotes && !inside_double_quotes {
            // backslashes preserve literal value of next character
            if backslash {
                arg.push(c);
                backslash = false;
                continue;
            }
            if c == '\\' {
                backslash = true;
                continue;
            }

            // double quotes inside single quotes do not have special meaning
            if c == '\'' {
                inside_single_quotes = true;
                continue;
            } else if c == '"' {
                inside_double_quotes = true;
                continue;
            }

            // skip white spaces
            if c.is_whitespace() {
                if !arg.is_empty() {
                    args.push(arg.clone());
                    arg.clear();
                }
                continue;
            }
        } else if inside_single_quotes && c == '\'' {
            inside_single_quotes = false;
            continue;
        } else if inside_double_quotes {
            // backslashes may have special meaning
            if backslash {
                backslash = false;
                if c == '$' || c == '"' || c == '\\' {
                    // apply special handling
                    // for now special handling is just pushing the character after backslash
                    // which means do nothing, as the character is pushed at the end of the loop
                } else {
                    // backslash is not followed by a special character
                    arg.push('\\');
                }
            } else if c == '\\' {
                backslash = true;
                continue;
            } else if c == '"' {
                inside_double_quotes = false;
                continue;
            }
        }
        arg.push(c);
    }

    if !arg.is_empty() {
        args.push(arg.clone());
    }

    args
}

pub fn parse_redirection<'a>(
    args: &'a Vec<String>,
) -> (Vec<&'a String>, Box<dyn Write>, Box<dyn Write>) {
    let mut cmd_args = Vec::new();
    let mut out: Box<dyn Write> = Box::new(std::io::stdout());
    let mut err: Box<dyn Write> = Box::new(std::io::stderr());

    let mut i = 0;
    while i < args.len() {
        if (args[i] == "1>" || args[i] == ">") && i + 1 < args.len() {
            let out_fname = &args[i + 1];
            if let Ok(file) = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&out_fname)
            {
                out = Box::new(BufWriter::new(file));
            } else {
                eprintln!("Failed to open output file in write mode {}", out_fname);
            }
            i += 1;
        } else if args[i] == "2>" && i + 1 < args.len() {
            let err_fname = &args[i + 1];
            if let Ok(file) = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&err_fname)
            {
                err = Box::new(BufWriter::new(file));
            } else {
                eprintln!("Failed to open error file in write mode {}", err_fname);
            }
            i += 1;
        } else if (args[i] == "1>>" || args[i] == ">>") && i + 1 < args.len() {
            let out_fname = &args[i + 1];
            if let Ok(file) = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&out_fname)
            {
                out = Box::new(BufWriter::new(file));
            } else {
                eprintln!("Failed to open output file in append mode {}", out_fname);
            }
            i += 1;
        } else if args[i] == "2>>" && i + 1 < args.len() {
            let err_fname = &args[i + 1];
            if let Ok(file) = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&err_fname)
            {
                err = Box::new(BufWriter::new(file));
            } else {
                eprintln!("Failed to open error file in append mode {}", err_fname);
            }
            i += 1;
        } else {
            cmd_args.push(&args[i]);
        }
        i += 1;
    }

    (cmd_args, out, err)
}
