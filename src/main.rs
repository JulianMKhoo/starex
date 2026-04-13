use std::io::{BufWriter, stdout};

mod cli;
mod config;
mod core;

fn main() {
    let std = stdout();
    let lock = BufWriter::new(std.lock());
    cli::cli::cli_entry(lock);
}
