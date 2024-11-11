use clap::{Parser, Subcommand};
use logging::logs;
mod logging;
use sysmain_lib::tempclean;

#[macro_use]
extern crate tracing;

fn cli_entry(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Command::Tempclean => {
            tempclean()?;
        }
    }
    Ok(())
}

fn main() {
    logs();
    let cli = Cli::parse();
    if let Err(why) = cli_entry(&cli) {
        error!("{why}");
    }
}

#[derive(Parser)]
#[command(version, about)]
/// CLI to manage system maintenance
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Clean temp files
    Tempclean,
}
