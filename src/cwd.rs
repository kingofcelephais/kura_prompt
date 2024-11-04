use nu_ansi_term::Color::*;
use std::env;

pub fn cwd() -> Option<String> {
    let path_env = env::current_dir().ok()?;
    let mut path = format!("{}", path_env.display());
    let home = env::var("HOME").unwrap();
    let tilde_expand = env::var("IAY_EXPAND_TILDE").unwrap_or_else(|_| "0".into());

    if let "0" = tilde_expand.as_ref() {
        let home_dir = &home;
        let home_dir_ext = format!("{}{}", home_dir, "/");
        if (&path == home_dir) || path.starts_with(&home_dir_ext) {
            path = path.replacen(&home_dir[..], "~", 1);
        }
    };

    Some(Red.paint(&path).to_string())
}

pub fn uname() -> Option<String> {
    let key = "USER";
    match env::var(key) {
        Ok(val) => Some(Red.paint(val).to_string()),
        Err(_) => Some("Error".to_string()),
    }
}
