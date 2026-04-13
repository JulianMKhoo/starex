use nix::{
    sys::stat::{Mode, umask},
    unistd::{ForkResult, chdir, fork, setsid},
};
use std::{
    fs::{self},
    process::{exit, id},
};

use crate::core::net::listen_term;

pub fn create_daemon() {
    umask(Mode::from_bits_truncate(0o000));

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => exit(0),
        Ok(ForkResult::Child) => {
            setsid().expect("SET SID FAILED");
            match unsafe { fork() } {
                Ok(ForkResult::Parent { .. }) => exit(0),
                Ok(ForkResult::Child) => {
                    chdir("/").expect("CHANGE DIR FAILED");

                    let pid = id();

                    fs::write("/tmp/starex_daemon.pid", pid.to_string())
                        .expect("PID WRITE FILE FAILED");

                    println!("DEAMON ACTIVD AT {:?}", pid);
                    println!(
                        "ENSURE YOU ALREADY HAVE `eval \"$(starex init)\"` AFTER STARSHIP EVAL LINE IN .zshrc THEN SOURCE IT."
                    );

                    listen_term();
                }
                Err(_) => panic!("FORK PROC FAILED"),
            }
        }
        Err(_) => panic!("FORK FAILED"),
    }
}
