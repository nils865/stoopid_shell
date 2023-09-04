use std::io::{stdin, stdout, Write};

mod commands;
use commands::input_handler;

fn main() {
    let running = true;
    let prompt = "➜  ";
    let mut exit_code: i8 = 0;

    while running {
        let mut command = String::new();

        print!("{}", prompt);
        stdout().flush().expect("Failed to flush stdout");
        stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let args: Vec<&str> = command.trim().split(" ").collect();

        exit_code = input_handler(args);

        if exit_code == -1 {
            break;
        }
    }
}
