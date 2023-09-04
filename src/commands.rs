use std::process::Command;

fn syscalls(args: Vec<&str>) -> i8 {
    let cmd_args = args.iter().skip(1).cloned().collect::<Vec<_>>();

    let mut cmd = Command::new(args[0]);

    if cmd_args.len() > 0 {
        cmd.arg(cmd_args.join(" "));
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
    match args[0] {
        "exit" => return -1,

        _ => return syscalls(args),
    }
}
