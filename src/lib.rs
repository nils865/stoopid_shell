use std::env;

pub fn get_folder_splitter() -> String {
    return match env::consts::OS {
        "windows" => String::from("\\"),
        _ => String::from("/"),
    };
}
