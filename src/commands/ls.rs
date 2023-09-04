use std::{env, fs, path::Path};

pub fn cmd_ls(args: &Vec<String>) -> i8 {
    let dir: String;

    if args.len() == 0 {
        dir = env::current_dir().unwrap().display().to_string();
    } else if args.len() == 1 {
        dir = args[0].clone();
    } else {
        println!("ls: Too many Arguments Provided");
        return 1;
    }

    let files = match fs::read_dir(Path::new(&dir)) {
        Ok(res) => res,
        Err(_) => {
            println!("ls: Directory \"{}\" not Found", dir);
            return 1;
        }
    };

    for file in files {
        println!(
            "{}",
            file.unwrap()
                .path()
                .display()
                .to_string()
                .split("/")
                .last()
                .unwrap()
        );
    }

    return 0;
}
