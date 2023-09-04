use colored::Colorize;

pub fn get_prompt(exit_code: i8) -> String {
    let mut prompt = String::new();

    if exit_code == 0 {
        prompt.push_str(&"➜ ".green().to_string());
    } else {
        prompt.push_str(&"➜ ".red().to_string());
    }

    return prompt;
}
