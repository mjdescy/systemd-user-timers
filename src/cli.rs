use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Args)]
pub struct AddCommand {
    /// the executable the timer will run
    #[arg(short, long)]
    pub exec: String,

    /// the schedule for the timer ("daily", "weekly", "monthly", "Year-Month-Day Hour:Minute:Second")
    #[arg(short, long)]
    pub when: String,

    /// name of the timer (optional); if omitted, the executable name will be used
    #[arg(short, long)]
    pub name: Option<String>,

    /// description of the timer (optional); if omitted, a description will be generated
    #[arg(short, long)]
    pub description: Option<String>,

    /// execute immediately if missed (default: true)
    #[arg(short = 'm', long, default_value_t = true)]
    pub exec_if_missed: bool,

    /// verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Args)]
pub struct NameCommand {
    /// name of the timer
    #[arg(short, long)]
    pub name: String,

    /// verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}   

#[derive(Subcommand)]
pub enum Commands {
    /// add a timer
    Add(AddCommand),
    /// enable a disabled timer
    Enable(NameCommand),
    /// disable an enabled timer
    Disable(NameCommand),
    /// start a timer
    Start(NameCommand),
    /// stop a timer
    Stop(NameCommand),
    /// remove a timer (deletes both timer and service files)
    Remove(NameCommand),
    /// show status of a user timer
    Status(NameCommand),
    /// list all user timers
    List {},
}
