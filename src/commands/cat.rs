use std::{fs::File, io::Read, path::Path};

use stoopid_shell::{syserr, sysout};

pub fn cmd_cat(args: &Vec<String>) -> i8 {
    if args.len() != 1 {
        return syserr("cp", "Usage: cat <file>");
    }

    return match File::open(Path::new(&args[0])) {
        Ok(mut f) => {
            let mut content = String::new();
            let _ = f.read_to_string(&mut content);

            sysout(&content);

            0
        }
        Err(_) => syserr("cat", format!("file \"{}\" not Found", args[0]).as_str()),
    };
}
