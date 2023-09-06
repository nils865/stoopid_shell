use std::env;

use colored::Colorize;

pub fn get_folder_splitter() -> String {
    return match env::consts::OS {
        "windows" => String::from("\\"),
        _ => String::from("/"),
    };
}

pub fn sysout(msg: &str) -> i8 {
    println!("{}", msg);

    return 0;
}

pub fn syserr(caller: &str, msg: &str) -> i8 {
    eprintln!("{}: {}", caller.bright_yellow(), msg.bright_red());

    return 1;
}
