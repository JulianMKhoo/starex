use crate::cli::cli_struct::{Cli, Cmd};
use crate::core::holiday::holiday::{
    get_full_holiday_logic, get_month_holiday_logic, get_now_holiday_logic,
};
use crate::core::init::print_init_script;
use crate::core::kill::kill_daemon;
use crate::core::start::create_daemon;
use crate::core::status::check_status;
use crate::core::time::time::get_time_logic;
use clap::Parser;
use std::io::{BufWriter, StdoutLock, Write};

pub fn cli_entry(mut lock: BufWriter<StdoutLock<'_>>) {
    let cli = Cli::parse();
    match &cli.command {
        Cmd::Start => create_daemon(),
        Cmd::Init => print_init_script(),
        Cmd::Kill => kill_daemon(),
        Cmd::Time {} => {
            writeln!(lock, "{}", get_time_logic()).unwrap();
        }
        Cmd::Holiday { now, month, full } => {
            if *now {
                writeln!(lock, "{}", get_now_holiday_logic()).unwrap();
            } else if let Some(m) = month {
                writeln!(lock, "{}", get_month_holiday_logic(*m)).unwrap();
            } else if *full {
                writeln!(lock, "{}", get_full_holiday_logic()).unwrap();
            } else {
                writeln!(lock, "{}", get_full_holiday_logic()).unwrap();
            }
        }
        Cmd::Weather => {}
        Cmd::Status => check_status(),
    }
}
