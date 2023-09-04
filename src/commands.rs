use std::process::Command;

use self::cd::cmd_cd;

mod cd;

fn args_handler(args: &Vec<&str>) -> Vec<String> {
    let mut i = 1;

    let mut cmd_args: Vec<String> = vec![];

    while i < args.len() {
        let mut arg: String = args[i].to_string();

        if arg.starts_with("\"") {
            i += 1;

            for j in i..args.len() {
                arg = format!("{} {}", arg, args[j]);

                if arg.ends_with("\"") {
                    break;
                }

                i += 1;
            }
        }

        if arg.starts_with("\"") {
            arg = arg.strip_prefix("\"").unwrap().to_string();
        }

        if arg.ends_with("\"") {
            arg = arg.strip_suffix("\"").unwrap().to_string();
        }

        cmd_args.push(arg);

        i += 1;
    }

    return cmd_args;
}

fn syscalls(args: Vec<&str>) -> i8 {
    let cmd_args = args_handler(&args);

    let mut cmd = Command::new(args[0]);

    if cmd_args.len() > 0 {
        cmd.args(&cmd_args);
    }

    return match cmd.status() {
        Ok(status) => {
            if status.success() {
                0
            } else {
                1
            }
        }
        Err(_) => {
            println!("Command not found: {}", args[0]);
            1
        }
    };
}

pub fn input_handler(args: Vec<&str>) -> i8 {
    return match args[0] {
        "cd" => cmd_cd(args),
        "exit" => -1,

        _ => syscalls(args),
    };
}
