use clap::{Command, arg, command, value_parser};

fn main() {

    let matches = command!()
    .subcommand(
        Command::new("add")
            .about("Add a timer")
            .args([
                arg!(-e --exec <EXECUTABLE> "The executable the timer will run")
                    .value_parser(value_parser!(String)),
                arg!(-m --"exec-if-missed" "Execute immediately if missed")
                    .action(clap::ArgAction::SetTrue)
            ])
    )
    .get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
        println!("Add subcommand exec-if-missed value: {}", add_matches.get_flag("exec-if-missed"));
    } else {
        println!("No subcommand")
    }
}
