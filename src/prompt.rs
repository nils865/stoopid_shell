use colored::Colorize;
use std::env;
use stoopid_shell::get_folder_splitter;

pub fn get_prompt(exit_code: i8) -> String {
    let mut prompt = String::new();

    if exit_code == 0 {
        prompt.push_str(&"➜  ".green().to_string());
    } else {
        prompt.push_str(&"➜  ".red().to_string());
    }

    let dir = env::current_dir().unwrap().display().to_string();

    let mut display_dir = dir.split(&get_folder_splitter()).last().unwrap();

    if display_dir == "" {
        display_dir = "/";
    } else if dir == env::var("HOME").unwrap_or_default() {
        display_dir = "~";
    }

    prompt.push_str(
        &format!("{} ", display_dir)
            .bright_cyan()
            .to_string()
            .clone(),
    );

    return prompt.bold().to_string();
}
