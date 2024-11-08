use nu_ansi_term::Color::*;
use std::env;

pub fn cwd() -> Option<String> {
    let path_env = env::current_dir().ok()?;
    let mut path = format!("{}", path_env.display());
    let key = "HOME";
    let home;
    match env::var(key) {
        Ok(val) => home = val,
        Err(_) => home = "".to_string(),
    }

    if path == home {
        path = "~".to_string();
        return Some(Red.paint(path).to_string());
    }

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
