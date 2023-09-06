use std::{
    io::{stdin, stdout, Write},
    process,
};

mod commands;
mod prompt;

use commands::input_handler;
use prompt::get_prompt;

fn main() {
    let running = true;
    let mut exit_code: i8 = 0;

    while running {
        let mut command = String::new();

        print!("{}", get_prompt(exit_code));
        stdout().flush().expect("Failed to flush stdout");
        stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let args: Vec<&str> = command.trim().split(" ").collect();

        exit_code = input_handler(args);

        if exit_code == -1 {
            process::exit(0);
        } else if exit_code == -2 {
            process::exit(1);
        }
    }
}
