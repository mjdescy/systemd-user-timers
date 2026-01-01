use std::path::PathBuf;
use shellexpand;

use crate::cli::{AddCommand, NameCommand};
use crate::usertimer::UserTimer;

/// Execute add timer command; add a new user timer
pub fn add_timer_command(add_cmd: &AddCommand) {
    if !validate_executable_path(&add_cmd.exec) {
        eprintln!("Error: Executable path does not point to a valid file: {}", add_cmd.exec);
        return;
    }

    if !validate_schedule(&add_cmd.when) {
        eprintln!("Error: Invalid schedule format: {}", add_cmd.when);
        return;
    }

    // Prepare timer parameters
    let timer_exec_path = shellexpand::tilde(&add_cmd.exec).to_string();    
    let timer_name = get_timer_name(&add_cmd.exec, &add_cmd.name);
    let timer_description = get_timer_description(&add_cmd.exec, &add_cmd.description);
    let timer_schedule = add_cmd.when.clone();

    // Create UserTimer instance, which will help generate file paths and contents
    let user_timer = UserTimer {
        executable: timer_exec_path,
        description: timer_description,
        schedule: timer_schedule,
        name: timer_name,
        exec_if_missed: add_cmd.exec_if_missed,
    };

    // Check if service file exists before creating timer file
    let service_path = user_timer.service_file_path();
    if !service_path.exists() {
        eprintln!("Error: Service file does not exist at {:?}", service_path);
        return;
    }

    // Create the service file
    if let Err(e) = std::fs::write(user_timer.service_file_path(), user_timer.service_file_contents()) {
        eprintln!("Error: Failed to create service file: {}", e);
        return;
    }

    // Create the timer file
    if let Err(e) = std::fs::write(user_timer.timer_file_path(), user_timer.timer_file_contents()) {
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
    reload_daemon();

    // Ensure timer is enabled and started
    enable_timer(&user_timer.name);
    start_timer(&user_timer.name);
}

/// Validate that the executable path points to a valid file
fn validate_executable_path(exec: &str) -> bool {
    // Extract just the command (first word) before any arguments
    let command = exec.split_whitespace().next().unwrap_or(exec);
    let full_path = shellexpand::tilde(command).to_string();
    
    // First check if it's a direct file path
    let exec_path = PathBuf::from(&full_path);
    if exec_path.is_file() {
        return true;
    }
    
    // If not, check if it's in PATH using 'which'
    std::process::Command::new("which")
        .arg(&full_path)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Validate the schedule format using systemd-analyze
fn validate_schedule(schedule: &str) -> bool {
    let output = std::process::Command::new("systemd-analyze")
        .arg("calendar")
        .arg(schedule)
        .output()
        .expect("Failed to execute systemd-analyze");
    output.status.success()
}

/// Get the timer name, defaulting to executable name if not provided
fn get_timer_name(exec: &str, name: &Option<String>) -> String {
    match name {
        Some(n) => n.to_string(),
        None => {
            // Extract just the command (first word) before any arguments
            let command = exec.split_whitespace().next().unwrap_or(exec);
            PathBuf::from(command)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(command)
                .replace('.', "_")
                .to_string()
        }
    }
}

/// Get the timer description, defaulting to "Execute {exec}" if not provided
fn get_timer_description(exec: &str, description: &Option<String>) -> String {
    match description {
        Some(d) => d.to_string(),
        None => format!("Execute {}", exec),
    }
}

/// Execute status command; show the status of a user timer
pub fn status_command(name_cmd: &NameCommand) {
    let output = std::process::Command::new("systemctl")
        .arg("--user")
        .arg("status")
        .arg(format!("{}.timer", name_cmd.name))
        .output()
        .expect("Failed to execute systemctl");

    if output.status.success() {
        let status = String::from_utf8_lossy(&output.stdout);
        println!("{}", status);
    } else {
        eprintln!("Error: Failed to get status for timer {}", name_cmd.name);
    }
}

/// Execute list timers command. List all user timers
pub fn list_timers_command() {
    // List all user timers using systemctl
    let output = std::process::Command::new("systemctl")
        .arg("--user")
        .arg("list-timers")
        .arg("--all")
        .output()
        .expect("Failed to execute systemctl");

    if output.status.success() {
        let timers = String::from_utf8_lossy(&output.stdout);
        println!("{}", timers);
    } else {
        eprintln!("Error: Failed to list timers");
    }
}

/// Execute enable timer command
pub fn enable_timer_command(name_cmd: &NameCommand) {
    enable_timer(&name_cmd.name);
}

/// Enable a timer by name
fn enable_timer(name: &str) {
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("enable")
        .arg("--now")
        .arg(format!("{}.timer", name))
        .status()
        .expect("Failed to enable the timer");
}

/// Execute disable timer command
pub fn disable_timer_command(name_cmd: &NameCommand) {
    disable_timer(&name_cmd.name);
}

/// Disable a timer by name
fn disable_timer(name: &str) {
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("disable")
        .arg("--now")
        .arg(format!("{}.timer", name))
        .status()
        .expect("Failed to disable the timer");
}

/// Execute start timer command
pub fn start_timer_command(name_cmd: &NameCommand) {
    start_timer(&name_cmd.name);
}

/// Start a timer by name
fn start_timer(name: &str) {
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("start")
        .arg(format!("{}.timer", name))
        .status()
        .expect("Failed to start the timer");
}

/// Execute stop timer command
pub fn stop_timer_command(name_cmd: &NameCommand) {
    stop_timer(&name_cmd.name);
}

/// Stop a timer by name
fn stop_timer(name: &str) {
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("stop")
        .arg(format!("{}.timer", name))
        .status()
        .expect("Failed to stop the timer");
}

/// Execute remove timer command
pub fn remove_timer_command(name_cmd: &NameCommand) {
    remove_timer(&name_cmd.name);
}

/// Remove a timer by name
fn remove_timer(name: &str) {
    // Disable the timer first
    disable_timer(name);

    // Remove the timer and service files
    let user_timer = UserTimer {
        executable: String::new(),
        description: String::new(),
        schedule: String::new(),
        name: name.to_string(),
        exec_if_missed: true,
    };

    if let Err(e) = std::fs::remove_file(user_timer.timer_file_path()) {
        eprintln!("Error: Failed to remove timer file: {}", e);
    }

    if let Err(e) = std::fs::remove_file(user_timer.service_file_path()) {
        eprintln!("Error: Failed to remove service file: {}", e);
    }

    // Reload systemd daemon
    reload_daemon();
}

/// Reload the systemd user daemon
fn reload_daemon() {
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("daemon-reload")
        .status()
        .expect("Failed to reload systemd daemon");
}