mod subcommand;
mod usertimer;
mod cli;

use clap::Parser;
use crate::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add(add_cmd)) => {
            subcommand::add_timer_command(add_cmd);
        },
        Some(Commands::Enable(name_cmd)) => {
            subcommand::enable_timer_command(&name_cmd);
        },
        Some(Commands::Disable(name_cmd)) => {
            subcommand::disable_timer_command(&name_cmd);
        },
        Some(Commands::Start(name_cmd)) => {
            subcommand::start_timer_command(&name_cmd);
        },
        Some(Commands::Stop(name_cmd)) => {
            subcommand::stop_timer_command(&name_cmd);
        },
        Some(Commands::Remove(name_cmd)) => {
            subcommand::remove_timer_command(&name_cmd);
        },
        Some(Commands::Status(name_cmd)) => {
            subcommand::status_command(&name_cmd);
        },
        Some(Commands::List {}) => {
            subcommand::list_timers_command();
        },
        None => {
            println!("No command provided. Use --help for usage information.");
        }
    }
}
