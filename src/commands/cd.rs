use std::{env, path::Path};

pub fn cmd_cd(args: &Vec<String>) -> i8 {
    if args.len() == 0 {
        env::set_current_dir(Path::new(env::var("HOME").unwrap().as_str())).unwrap();
        return 0;
    }

    match env::set_current_dir(Path::new(args[0].as_str())) {
        Ok(_) => return 0,
        Err(_) => {
            println!("cd: {}: No such file or directory", args[0]);
            return 1;
        }
    }
}
