use std::io::{stdin, stdout, Write};

fn main() {
    let running = true;
    let prompt = "➜  ";

    while running {
        let mut command = String::new();

        print!("{}", prompt);
        stdout().flush().expect("Failed to flush stdout");
        stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let args: Vec<&str> = command.trim().split(" ").collect();

        match args[0] {
            "exit" => return,
            _ => println!("Command not found: {}", args[0]),
        }
    }
}
