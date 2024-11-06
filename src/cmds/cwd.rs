use nu_ansi_term::Color::*;
use std::env;

pub fn cwd() -> Option<String> {
    let path_env = env::current_dir().ok()?;
    let path = format!("{}", path_env.display());

    let split_path = path.split("/").last().unwrap_or_default();

    Some(Red.paint(split_path).to_string())
}

pub fn uname() -> Option<String> {
    let key = "USER";
    match env::var(key) {
        Ok(val) => Some(Red.paint(val).to_string()),
        Err(_) => Some("Error".to_string()),
    }
}
