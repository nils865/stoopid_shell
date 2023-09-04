use colored::Colorize;
use std::env;

pub fn get_prompt(exit_code: i8) -> String {
    let mut prompt = String::new();

    if exit_code == 0 {
        prompt.push_str(&"➜ ".green().to_string());
    } else {
        prompt.push_str(&"➜ ".red().to_string());
    }

    prompt.push_str(
        &format!(
            "{} ",
            env::current_dir()
                .expect("Failed to get current directory")
                .display()
                .to_string()
                .split("/")
                .last()
                .unwrap()
        )
        .bright_cyan()
        .to_string(),
    );

    return prompt.bold().to_string();
}
