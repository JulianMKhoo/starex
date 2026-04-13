use std::{fs, process::Command};

pub fn kill_daemon() {
    if let Ok(pid_str) = fs::read_to_string("/tmp/starex_daemon.pid") {
        let pid = pid_str.trim();
        let _ = Command::new("kill").arg(&pid).status().ok();
        let _ = fs::remove_file("/tmp/starex_daemon.pid");
        let _ = fs::remove_file("/tmp/starex.sock");

        println!("KILL {:?}", pid);
    } else {
        println!("CANNOT READ THE PID FILE");
    }
}
