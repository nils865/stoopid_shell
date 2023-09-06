use stoopid_shell::syserr;

use std::{fs::File, path::Path};

pub fn cmd_touch(args: &Vec<String>) -> i8 {
    if args.len() != 1 {
        return syserr("touch", "Usage: touch <name>");
    }

    return match File::create(Path::new(&args[0])) {
        Ok(_) => 0,
        Err(e) => return syserr("touch", &e.to_string()),
    };
}
