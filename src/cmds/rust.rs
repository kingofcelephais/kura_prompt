use nu_ansi_term::Color::*;
use std::fs;
use std::io;
use toml::Table;

pub fn get_path() -> io::Result<String> {
    let mut check = false;
    let mut path_string: String = ".".to_string();
    let mut depth = 0;
    loop {
        let entries = fs::read_dir(path_string.clone())?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.ends_with("Cargo.toml") {
                let file_path = path.to_string_lossy().to_string();
                return Ok(file_path);
            }
        }
        if path_string == "." {
            path_string = "..".to_string();
        } else {
            path_string = path_string.clone().to_owned() + "/..";
        }
        depth += 1;
        if depth == 10 {
            check = true;
        }
        if check == true {
            break;
        }
    }

    Ok("".to_string())
}

pub fn rust() -> Option<String> {
    let path = get_path().unwrap_or_default();
    let contents = fs::read_to_string(path).expect("file error");

    let toml_contents = contents.parse::<Table>().unwrap();
    let name = toml_contents["package"]["name"].clone();
    let num = toml_contents["package"]["version"].clone();

    let fin = name.as_str().unwrap_or_default().to_owned() + " " + num.as_str().unwrap_or_default();

    Some(Red.paint(fin).to_string())
}
