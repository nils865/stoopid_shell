use stoopid_shell::syserr;

pub fn cmd_exit(args: &Vec<String>) -> i8 {
    return match args.len() {
        0 => -1,
        1 => match args[0].as_str() {
            "0" => -1,
            "1" => -2,
            _ => syserr("exit", "Invalid exit code"),
        },
        _ => syserr("exit", "Too many Arguments"),
    };
}
