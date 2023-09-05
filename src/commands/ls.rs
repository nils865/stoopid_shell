use colored::Colorize;
use std::{env, fs, path::Path};
use stoopid_shell::get_folder_splitter;

pub fn cmd_ls(args: &Vec<String>) -> i8 {
    let dir: String;

    if args.len() == 0 {
        dir = env::current_dir().unwrap_or_default().display().to_string();
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
        let path = match file {
            Ok(f) => f.path(),
            Err(_) => continue,
        };

        let name = String::from(
            path.display()
                .to_string()
                .split(&get_folder_splitter())
                .last()
                .unwrap_or_default(),
        );

        if path.is_dir() {
            println!("{}", name.bright_cyan().bold());
        } else {
            println!("{}", name);
        }
    }

    return 0;
}
