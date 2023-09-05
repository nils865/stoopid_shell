use std::env;

use colored::Colorize;

pub fn get_folder_splitter() -> String {
    return match env::consts::OS {
        "windows" => String::from("\\"),
        _ => String::from("/"),
    };
}

pub fn syserr(caller: &str, msg: &str) {
    eprintln!("{}: {}", caller.bright_yellow(), msg.bright_red());
}
