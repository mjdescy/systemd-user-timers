use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Args)]
struct AddCommand {
    /// the executable the timer will run
    #[arg(short, long)]
    exec: String,

    /// the schedule for the timer ("daily", "weekly", "monthly", "Year-Month-Day Hour:Minute:Second")
    #[arg(short, long)]
    when: String,

    /// name of the timer (optional); if omitted, the executable name will be used
    #[arg(short, long)]
    name: Option<String>,

    /// description of the timer (optional); if omitted, a description will be generated
    #[arg(short, long)]
    description: Option<String>,

    /// execute immediately if missed (default: true)
    #[arg(short = 'm', long, default_value_t = true)]
    exec_if_missed: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// add a timer
    Add(AddCommand),
    /// enable a disabled timer
    Enable {
        /// name of the timer to enable
        name: String,
    },
    /// disable an enabled timer
    Disable {
        /// name of the timer to disable
        name: String,
    },
    /// start a timer
    Start {
        /// name of the timer to enable
        name: String,
    },
    /// stop a timer
    Stop {
        /// name of the timer to stop
        name: String,
    },
    /// remove a timer (deletes both timer and service files)
    Remove {
        /// name of the timer to remove
        name: String,
    },
    /// list all user timers
    List {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add(add_cmd)) => {
            println!("Adding a timer with exec: {}, when: {}", add_cmd.exec, add_cmd.when);
        },
        Some(Commands::Enable { name }) => {
            println!("Enabling timer: {}", name);
        },
        Some(Commands::Disable { name }) => {
            println!("Disabling timer: {}", name);
        },
        Some(Commands::Start { name }) => {
            println!("Starting timer: {}", name);
        },
        Some(Commands::Stop { name }) => {
            println!("Stopping timer: {}", name);
        },
        Some(Commands::Remove { name }) => {
            println!("Removing timer: {}", name);
        },
        Some(Commands::List {}) => {
            println!("Listing all user timers");
        },
        None => {}
    }
}
