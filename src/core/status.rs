use std::{env, fs};

pub fn check_status() {
    let home = env::var("HOME").expect("HOME NULL");
    let zshrc_path = format!("{}/.zshrc", home);
    let zshrc_content = fs::read_to_string(&zshrc_path).unwrap_or_default();
    if let Ok(pid_str) = fs::read_to_string("/tmp/starex_daemon.pid") {
        let pid = pid_str.trim();
        println!("DAEMON ACTIVE AT {:?}", pid);
    } else {
        panic!("DAEMON IS INACTIVE TO START `starex start`");
    }
    if !zshrc_content.contains("starex init") {
        println!(
            "WARNING FORGOT TO ADD `eval \"$(starex init)\"` AFTER STARSHIP EVAL LINE IN .zshrc AND THEN SOURCE IT."
        );
    }
}
