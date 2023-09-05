use std::env;

pub fn cmd_echo(args: &Vec<String>) -> i8 {
    let mut text: String = String::new();

    for arg in args {
        let mut val: String = arg.clone();

        if arg.starts_with("$") {
            val = env::var(arg.strip_prefix("$").unwrap_or("")).unwrap_or(String::new());
        }

        text = String::from(format!("{} {}", text, val));
    }

    println!("{}", text);

    return 0;
}
