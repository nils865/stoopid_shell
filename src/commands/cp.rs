use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use stoopid_shell::syserr;

pub fn cmd_cp(args: &Vec<String>) -> i8 {
    if args.len() != 2 {
        return syserr("cp", "Usage: cp <file> <destination>");
    }

    let file: String = match File::open(Path::new(&args[0])) {
        Ok(mut f) => {
            let mut content = String::new();

            let _ = f.read_to_string(&mut content);

            content
        }
        Err(_) => return syserr("cp", format!("file \"{}\" not Found", args[0]).as_str()),
    };

    let mut new_file = match File::create(Path::new(&args[1])) {
        Ok(f) => f,
        Err(e) => return syserr("cp", &e.to_string()),
    };

    let _ = new_file.write_all(file.as_bytes());

    return 0;
}
