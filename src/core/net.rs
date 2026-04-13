use std::{fs, io::Write, os::unix::net::UnixListener, thread, time::Duration};

use crate::core::time::time::get_time_logic;

const SOCKET_PATH: &str = "/tmp/starex.sock";

pub fn listen_term() {
    if fs::metadata(SOCKET_PATH).is_ok() {
        let _ = fs::remove_file(SOCKET_PATH);
    }
    let listener = UnixListener::bind(SOCKET_PATH).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    loop {
                        let message = format!("{}", get_time_logic());
                        if let Err(_) = writeln!(stream, "{}", message) {
                            break;
                        }
                        thread::sleep(Duration::from_secs(1));
                    }
                });
            }
            Err(_) => break,
        }
    }
}
