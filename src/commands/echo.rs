use std::env;

use stoopid_shell::sysout;

pub fn cmd_echo(args: &Vec<String>) -> i8 {
    let mut text: String = String::new();

    for arg in args {
        let mut val: String = arg.clone();

        if arg.starts_with("$") {
            val = env::var(arg.strip_prefix("$").unwrap_or("")).unwrap_or_default();
        }

        if text.is_empty() {
            text = val;
        } else {
            text = String::from(format!("{} {}", text, val));
        }
    }

    sysout(&text);

    return 0;
}
