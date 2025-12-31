mod command_add;
mod usertimer;

use crate::command_add::add_timer;

use clap::{Command, arg, command, value_parser};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("add")
                .about("Add a timer")
                .args([
                    arg!(-e --exec <EXECUTABLE> "The executable the timer will run")
                        .value_parser(value_parser!(String))
                        .required(true),
                    arg!(-m --"exec-if-missed" "Execute immediately if missed")
                        .action(clap::ArgAction::SetTrue),
                    arg!(-d --desc <DESCRIPTION> "A description of the timer")
                        .value_parser(value_parser!(String)),
                    arg!(-s --schedule <SCHEDULE> "The schedule for the timer")
                        .value_parser(value_parser!(String))
                        .required(true),
                    arg!(-n --name <NAME> "Optional: The name for the timer")
                        .value_parser(value_parser!(String)),
                    arg!(--"repeating" "Whether the timer is repeating")
                        .action(clap::ArgAction::SetTrue),
                    arg!(--"single-use" "Whether the timer is single-use")
                        .action(clap::ArgAction::SetTrue),
                    arg!(--"normal-service" "Whether the timer activates a normal service instead of a one-shot")
                        .action(clap::ArgAction::SetTrue)
                ])
        ).get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
        add_timer(add_matches);
    } else {
        println!("No subcommand");
    }
}
