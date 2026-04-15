use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "starex",
    version,
    about = "Starship Extra with real-time feature"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    Start,
    #[command(hide = true)]
    Init {
        shell_type: String,
    },
    Kill,
    Status,
    Time,
    Holiday {
        #[arg(short, long)]
        now: bool,
        #[arg(short, long, num_args = 0..=1, default_missing_value = "0")]
        month: Option<u32>,
        #[arg(short, long)]
        full: bool,
    },
    Weather,
}
