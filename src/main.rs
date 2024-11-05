mod cmds;

use crate::cmds::cwd;
use crate::cmds::rust;

use nu_ansi_term::Color::*;

fn main() {
    println!("{}", my_prompt());
}

fn my_prompt() -> String {
    let cwd = match cwd::cwd() {
        Some(c) => c,
        None => Red.paint("[directory does not exist]").to_string(),
    };
    let uname = match cwd::uname() {
        Some(u) => u,
        None => Red.paint("uname failed").to_string(),
    };
    let rust = match rust::rust() {
        Some(r) => r,
        None => "".to_string(),
    };

    format!(
        "[{uname}] <-> [{cwd}] <-> [{rust}] ",
        cwd = cwd,
        uname = uname,
        rust = rust
    )
}
