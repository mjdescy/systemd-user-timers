use clap::ArgMatches;
use std::path::PathBuf;
use shellexpand::tilde;

use crate::usertimer::UserTimer;

pub fn add_timer(add_matches: &ArgMatches) {
    // Extract executable and schedule (required arguments); return error if missing
    let Some(executable) = add_matches.get_one::<String>("exec") else {
        eprintln!("Error: Missing required argument 'exec'");
        return;
    };
    let Some(schedule) = add_matches.get_one::<String>("schedule") else {
        eprintln!("Error: Missing required argument 'schedule'");
        return;
    };

    // Check if the executable path points to a valid file
    let full_path = tilde(&executable).to_string();
    let exec_path = PathBuf::from(full_path);
    if !exec_path.is_file() {
        eprintln!("Error: Executable path does not point to a valid file: {}", executable);
        return;
    }
    
    // Check if the schedule is valid (via systemd-analyze)
    let schedule_check = std::process::Command::new("systemd-analyze")
        .arg("calendar")
        .arg(schedule)
        .output()
        .expect("Failed to execute systemd-analyze");

    if !schedule_check.status.success() {
        eprintln!("Error: Invalid schedule format: {}", schedule);
        return;
    }

    // The description is optional
    // If not provided, generate a default description from the executable
    let description_from_executable = format!("Execute {}", executable);

    // Extract description from the optional description argument or use description we generated
    // from the executable value
    let description = add_matches
        .get_one::<String>("desc")
        .map(|s| s.to_string())
        .unwrap_or_else(|| description_from_executable.to_string());

    // The name is optional
    // If not provided, generate a name from the executable
    // Use the executable name without path or file extension and replace dots with underscores
    let name_from_executable = PathBuf::from(executable)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(executable)
        .replace('.', "_");

    // Extract name from the optional name argument or use the name we generated from the executable value
    let name = add_matches
        .get_one::<String>("name")
        .map(|s| s.to_string())
        .unwrap_or(name_from_executable);

    // Create a struct for the timer parameters
    let user_timer = UserTimer {
        executable: exec_path.to_string_lossy().to_string(),
        description: description,
        schedule: schedule.to_string(),
        name: name,
        exec_if_missed: add_matches.get_flag("exec-if-missed")
    };

    // Create the service file
    if let Err(e) = user_timer.create_service_file() {
        eprintln!("Error: Failed to create service file: {}", e);
        return;
    }

    // Check if service file exists before creating timer file
    let service_path = user_timer.service_file_path();
    if !service_path.exists() {
        eprintln!("Error: Service file does not exist at {:?}", service_path);
        return;
    }

    // Create the timer file
    if let Err(e) = user_timer.create_timer_file() {
        eprintln!("Error: Failed to create timer file: {}", e);
        return;
    }

    // Check if timer file exists before proceeding
    let timer_path = user_timer.timer_file_path();
    if !timer_path.exists() {
        eprintln!("Error: Timer file does not exist at {:?}", timer_path);
        return;
    }

    // Reload systemd daemon
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("daemon-reload")
        .status()
        .expect("Failed to reload systemd daemon");

    // Enable and start the timer
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("enable")
        .arg("--now")
        .arg(format!("{}.timer", user_timer.name))
        .status()
        .expect("Failed to enable and start the timer");
}
