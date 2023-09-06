use std::process;

mod commands;
mod prompt;

use commands::input_handler;
use prompt::get_prompt;
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() {
    let running = true;
    let mut rl = DefaultEditor::new().expect("Failed to create readline");

    /*
    * 0: Success
    ! 1: Error
    * -1: Exit shell with Success
    ! -2: Exit shell with Error code
    */
    let mut exit_code: i8 = 0;

    while running {
        let command: String = match rl.readline(&get_prompt(exit_code)) {
            Ok(l) => {
                let _ = rl.add_history_entry(l.as_str());
                l
            }
            Err(ReadlineError::Interrupted) => String::new(),
            Err(ReadlineError::Eof) => process::exit(0),
            Err(_) => String::new(),
        };

        let args: Vec<&str> = command.trim().split(" ").collect();

        exit_code = input_handler(args);

        if exit_code == -1 {
            process::exit(0);
        } else if exit_code == -2 {
            process::exit(1);
        }
    }
}
