use std::process::Command;

use self::cd::cmd_cd;

mod cd;

fn syscalls(args: Vec<&str>) -> i8 {
    let mut cmd_args: Vec<String> = vec![];

    let mut i = 1;

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

        cmd_args.push(arg);

        i += 1;
    }

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
