use std::{fs, path::Path};

use stoopid_shell::syserr;

pub fn cmd_rm(args: &Vec<String>) -> i8 {
    if args.len() != 1 {
        return syserr("rm", "Usage: rm <file>");
    }

    return match fs::remove_file(Path::new(&args[0])) {
        Ok(_) => 0,
        Err(_) => syserr("rm", format!("File \"{}\" doesn't exist", args[0]).as_str()),
    };
}
