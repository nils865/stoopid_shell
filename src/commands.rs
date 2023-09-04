pub fn input_handler(args: Vec<&str>) -> i8 {
    match args[0] {
        "exit" => return -1,

        _ => {
            println!("Command not found: {}", args[0]);
            return 1;
        }
    }
}
