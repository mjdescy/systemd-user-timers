use std::path::PathBuf;
use shellexpand;

use crate::cli::{AddCommand, NameCommand, RemoveCommand, VerboseCommand};
use crate::usertimer::UserTimer;

/// Print message if verbose flag is set
fn verbose_print(verbose: bool, message: &str) {
    if verbose {
        println!("{}", message);
    }
}

/// Execute add timer command; add a new user timer
pub fn add_timer_command(add_cmd: &AddCommand) {
    let verbose = add_cmd.verbose;

    verbose_print(verbose, "Adding timer with the following parameters:");
    verbose_print(verbose, &format!("  Executable: {}", add_cmd.exec));
    verbose_print(verbose, &format!("  Schedule: {}", add_cmd.when));
    verbose_print(verbose, &format!("  Name: {:?}", add_cmd.name));
    verbose_print(verbose, &format!("  Description: {:?}", add_cmd.description));
    verbose_print(verbose, &format!("  Execute if missed: {}", add_cmd.exec_if_missed));
    verbose_print(verbose, "");

    verbose_print(verbose, "Validating executable path...");
    if !validate_executable_path(&add_cmd.exec) {
        eprintln!("Error: Executable path does not point to a valid file: {}", add_cmd.exec);
        return;
    }
    verbose_print(verbose, "Executable path is valid.");
    
    verbose_print(verbose, "Validating schedule format...");
    if !validate_schedule(&add_cmd.when) {
        eprintln!("Error: Invalid schedule format: {}", add_cmd.when);
        return;
    }
    verbose_print(verbose, "Schedule format is valid.");

    verbose_print(verbose, "Preparing timer parameters...");
    let timer_name = get_timer_name(&add_cmd.exec, &add_cmd.name);
    let full_timer_name = add_dot_timer_extension(&timer_name);
    let user_timer = UserTimer {
        executable: shellexpand::tilde(&add_cmd.exec).to_string(),
        description: get_timer_description(&add_cmd.exec, &add_cmd.description),
        schedule: add_cmd.when.clone(),
        name: full_timer_name,
        exec_if_missed: add_cmd.exec_if_missed,
    };

    // Create systemd user directory if it doesn't exist
    let systemd_dir = user_timer.systemd_dir();
    if !systemd_dir.exists() {
        verbose_print(verbose, &format!("Creating systemd user directory at {:?}", systemd_dir));
        if let Err(e) = std::fs::create_dir_all(&systemd_dir) {
            eprintln!("Error: Failed to create systemd user directory: {}", e);
            return;
        }
        if let Err(e) = std::fs::create_dir_all(&systemd_dir) {
            eprintln!("Error: Failed to create systemd user directory: {}", e);
            return;
        }
        verbose_print(verbose, "Created systemd user directory");
    }

    // Create the service file
    verbose_print(verbose, &format!("Creating service file at {:?}", user_timer.service_file_path()));
    if let Err(e) = std::fs::write(user_timer.service_file_path(), user_timer.service_file_contents()) {
        eprintln!("Error: Failed to create service file: {}", e);
        return;
    }
    verbose_print(verbose, "Created service file");

    // Create the timer file
    verbose_print(verbose, &format!("Creating timer file at {:?}", user_timer.timer_file_path()));
    if let Err(e) = std::fs::write(user_timer.timer_file_path(), user_timer.timer_file_contents()) {
        eprintln!("Error: Failed to create timer file: {}", e);
        return;
    }
    verbose_print(verbose, "Created timer file");

    // Reload systemd daemon
    verbose_print(verbose, "Reloading systemd user daemon...");
    reload_daemon();
    verbose_print(verbose, "Reloaded systemd user daemon");

    // Ensure timer is enabled and started
    verbose_print(verbose, &format!("Enabling timer {}", user_timer.name));
    enable_timer(&user_timer.name);
    verbose_print(verbose, &format!("Starting timer {}", user_timer.name));
    start_timer(&user_timer.name);
    verbose_print(verbose, "Timer added and started successfully.");
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

/// Ensure the timer name ends with ".timer" and never ends with ".timer.timer"
fn add_dot_timer_extension(name: &str) -> String {
    if name.ends_with(".timer") {
        name.to_string()
    } else {
        format!("{}.timer", name)
    }
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

/// Get the timer description, defaulting to "Execute {exec}" if not provided
fn get_timer_description(exec: &str, description: &Option<String>) -> String {
    match description {
        Some(d) => d.to_string(),
        None => format!("Execute {}", exec),
    }
}

/// Execute status command; show the status of a user timer
pub fn status_command(name_cmd: &NameCommand) {
    let verbose = name_cmd.verbose;
    let full_timer_name = add_dot_timer_extension(&name_cmd.name);
    
    verbose_print(verbose, &format!("Getting status for timer: {}...\n", full_timer_name));
    verbose_print(verbose, &format!("Equivalent command: systemctl --user status {}", full_timer_name));
    let output = std::process::Command::new("systemctl")
        .arg("--user")
        .arg("status")
        .arg(&full_timer_name)
        .output()
        .expect("Failed to execute systemctl");

    if output.status.success() {
        let status = String::from_utf8_lossy(&output.stdout);
        println!("{}", status);
    } else {
        eprintln!("Error: Failed to get status for timer {}", full_timer_name);
    }
}

/// Execute list timers command. List all user timers
pub fn list_timers_command(verbose_cmd: &VerboseCommand) {
    let verbose = verbose_cmd.verbose;
    
    // List all user timers using systemctl
    verbose_print(verbose, "Listing all user timers...");
    verbose_print(verbose, "Equivalent command: systemctl --user list-timers --all\n");
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
    let verbose = name_cmd.verbose;
    let full_timer_name = add_dot_timer_extension(&name_cmd.name);

    verbose_print(verbose, &format!("Enabling timer: {}", full_timer_name));
    verbose_print(verbose, &format!("Equivalent command: systemctl --user enable --now {}", full_timer_name));
    enable_timer(&full_timer_name);
}

/// Enable a timer by name
fn enable_timer(full_timer_name: &str) {
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("enable")
        .arg("--now")
        .arg(full_timer_name)
        .status()
        .expect("Failed to enable the timer");
}

/// Execute disable timer command
pub fn disable_timer_command(name_cmd: &NameCommand) {
    let verbose = name_cmd.verbose;
    let full_timer_name = add_dot_timer_extension(&name_cmd.name);

    verbose_print(verbose, &format!("Disabling timer: {}...", full_timer_name));
    verbose_print(verbose, &format!("Equivalent command: systemctl --user disable --now {}", full_timer_name));
    if let Err(e) = disable_timer(&full_timer_name) {
        eprintln!("{}", e);
    }
}

/// Disable a timer by name
fn disable_timer(full_timer_name: &str) -> Result<(), std::io::Error> {
    let status = std::process::Command::new("systemctl")
        .arg("--user")
        .arg("disable")
        .arg("--now")
        .arg(full_timer_name)
        .status()?;
    
    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to disable timer {}", full_timer_name)
        ));
    }
    
    Ok(())
}

/// Execute start timer command
pub fn start_timer_command(name_cmd: &NameCommand) {
    let verbose = name_cmd.verbose;
    let full_timer_name = add_dot_timer_extension(&name_cmd.name);

    verbose_print(verbose, &format!("Starting timer: {}...", full_timer_name));
    verbose_print(verbose, &format!("Equivalent command: systemctl --user start {}", full_timer_name));
    start_timer(&full_timer_name);
}

/// Start a timer by name
fn start_timer(full_timer_name: &str) {
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("start")
        .arg(full_timer_name)
        .status()
        .expect("Failed to start the timer");
}

/// Execute stop timer command
pub fn stop_timer_command(name_cmd: &NameCommand) {
    let verbose = name_cmd.verbose;
    let full_timer_name = add_dot_timer_extension(&name_cmd.name);

    verbose_print(verbose, &format!("Stopping timer: {}...", full_timer_name));
    verbose_print(verbose, &format!("Equivalent command: systemctl --user stop {}", full_timer_name));
    stop_timer(&full_timer_name);
}

/// Stop a timer by name
fn stop_timer(full_timer_name: &str) {
    std::process::Command::new("systemctl")
        .arg("--user")
        .arg("stop")
        .arg(full_timer_name)
        .status()
        .expect("Failed to stop the timer");
}

/// Execute remove timer command
pub fn remove_timer_command(name_cmd: &RemoveCommand) {
    let verbose = name_cmd.verbose;
    let full_timer_name = add_dot_timer_extension(&name_cmd.name);

    verbose_print(verbose, &format!("Removing timer: {}...", full_timer_name));
    verbose_print(verbose, &format!("Equivalent command: systemctl --user disable --now {}", full_timer_name));
    remove_timer(&full_timer_name, name_cmd.remove_service);
}

/// Remove a timer by name
fn remove_timer(full_timer_name: &str, remove_service: bool) {
    // Disable the timer first
    if let Err(e) = disable_timer(&full_timer_name) {
        eprintln!("Error: Failed to disable timer: {}", e);
        return;
    }

    let user_timer = UserTimer {
        executable: String::new(),
        description: String::new(),
        schedule: String::new(),
        name: full_timer_name.to_string(),
        exec_if_missed: true,
    };

    // Remove the timer file
    if let Err(e) = std::fs::remove_file(user_timer.timer_file_path()) {
        eprintln!("Error: Failed to remove timer file: {}", e);
    }

    // Remove the service file if requested
    if remove_service {
        if let Err(e) = std::fs::remove_file(user_timer.service_file_path()) {
            eprintln!("Error: Failed to remove service file: {}", e);
        }
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
