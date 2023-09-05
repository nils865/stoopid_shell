use std::{env, path::Path};
use stoopid_shell::syserr;

pub fn cmd_cd(args: &Vec<String>) -> i8 {
    if args.len() == 0 {
        env::set_current_dir(Path::new(env::var("HOME").unwrap_or_default().as_str()))
            .unwrap_or_default();
        return 0;
    } else if args.len() > 1 {
        syserr("cd", "Too many Arguments");
        return 1;
    }

    match env::set_current_dir(Path::new(args[0].as_str())) {
        Ok(_) => return 0,
        Err(_) => {
            syserr(
                "cd",
                format!("Directory \"{}\" not Found", args[0]).as_str(),
            );
            return 1;
        }
    }
}
