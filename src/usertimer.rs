use std::env;
use std::path::PathBuf;

/// Struct representing a user timer
pub struct UserTimer {
    pub executable: String,
    pub description: String,
    pub schedule: String,
    pub name: String,
    pub exec_if_missed: bool,
}

impl UserTimer {
    /// Get the systemd user directory path
    pub fn systemd_dir(&self) -> PathBuf {
        let home = env::var("HOME").expect("HOME environment variable not set");
        PathBuf::from(home).join(".config/systemd/user")
    }

    /// Get the name of the timer without the .timer suffix, if present
    pub fn base_name(&self) -> &str {
        if self.name.ends_with(".timer") {
            &self.name[..self.name.len() - 6]
        } else {
            &self.name
        }
    }

    /// Get the name of the service file
    pub fn service_file_name(&self) -> String {
        format!("{}.service", self.base_name())
    }

    /// Get the full path for the service file
    pub fn service_file_path(&self) -> PathBuf {
        self.systemd_dir()
            .join(self.service_file_name())
    }

    /// Generate the contents of the service file
    pub fn service_file_contents(&self) -> String {
        format!(
            "\
[Unit]
Description={description}

[Service]
Type=oneshot
ExecStart={executable}
",
            description = self.description,
            executable = self.executable
        )
    }

    /// Get the name of the timer file
    pub fn timer_file_name(&self) -> String {
        format!("{}.timer", self.base_name())
    }

    /// Get the full path for the timer file
    pub fn timer_file_path(&self) -> PathBuf {
        self.systemd_dir()
            .join(self.timer_file_name())
    }        
    
    /// Generate the contents of the timer file
    pub fn timer_file_contents(&self) -> String {
        format!(
            "\
[Unit]
Description={description}

[Timer]
OnCalendar={schedule}
{persistent}

[Install]
WantedBy=timers.target
",
            description = self.description,
            schedule = self.schedule,
            persistent = if self.exec_if_missed { "Persistent=true" } else { "" }
        )
    }
}
