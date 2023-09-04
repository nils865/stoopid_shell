use std::io::{stdin, stdout, Write};

fn main() {
    let running = true;
    let prompt = "➜  ";

    while running {
        let mut user_input = String::new();

        print!("{}", prompt);
        stdout().flush().expect("Failed to flush stdout");
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
    }
}
