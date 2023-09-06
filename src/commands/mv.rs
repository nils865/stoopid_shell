use std::{fs, path::Path};

use stoopid_shell::syserr;

pub fn cmd_mv(args: &Vec<String>) -> i8 {
    if args.len() != 2 {
        return syserr("mv", "Usage: mv <file> <destination>");
    }

    return match fs::rename(Path::new(&args[0]), Path::new(&args[1])) {
        Ok(_) => 0,
        Err(_) => syserr("mv", "Couldn't move file"),
    };
}
