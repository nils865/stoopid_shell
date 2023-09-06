use std::process::Command;
use stoopid_shell::syserr;

mod cd;
mod cp;
mod echo;
mod exit;
mod ls;
mod touch;

use self::{cd::cmd_cd, cp::cmd_cp, echo::cmd_echo, exit::cmd_exit, ls::cmd_ls, touch::cmd_touch};

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
            arg = arg.strip_prefix("\"").unwrap_or_default().to_string();
        }

        if arg.ends_with("\"") {
            arg = arg.strip_suffix("\"").unwrap_or_default().to_string();
        }

        cmd_args.push(arg);

        i += 1;
    }

    return cmd_args;
}

fn syscalls(command: &str, args: &Vec<String>) -> i8 {
    let mut cmd = Command::new(command);

    if args.len() > 0 {
        cmd.args(args);
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
            syserr(
                "stoopid_shell",
                format!("Command not found: {}", command).as_str(),
            );
            1
        }
    };
}

pub fn input_handler(args: Vec<&str>) -> i8 {
    let command = args[0];
    let arguments = args_handler(&args);

    return match command {
        "cd" => cmd_cd(&arguments),
        "ls" => cmd_ls(&arguments),
        "echo" => cmd_echo(&arguments),
        "exit" => cmd_exit(&arguments),
        "touch" => cmd_touch(&arguments),
        "cp" => cmd_cp(&arguments),
        "" => 0,

        _ => syscalls(&command, &arguments),
    };
}
