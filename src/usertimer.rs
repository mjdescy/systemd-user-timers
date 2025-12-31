use std::env;
use std::path::PathBuf;
use std::io;

pub struct UserTimer {
    pub executable: String,
    pub description: String,
    pub schedule: String,
    pub name: String,
    pub exec_if_missed: bool,
}

impl UserTimer {
    fn systemd_dir(&self) -> PathBuf {
        let home = env::var("HOME").expect("HOME environment variable not set");
        PathBuf::from(home).join(".config/systemd/user")
    }

    pub fn service_file_path(&self) -> PathBuf {
        PathBuf::from(self.systemd_dir())
            .join(format!("{}.service", self.name))
    }

    pub fn timer_file_path(&self) -> PathBuf {
        PathBuf::from(self.systemd_dir())
            .join(format!("{}.timer", self.name))
    }

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

    pub fn create_service_file(&self) -> io::Result<()> {
        std::fs::write(self.service_file_path(), self.service_file_contents())
    }

    pub fn create_timer_file(&self) -> io::Result<()> {
        std::fs::write(self.timer_file_path(), self.timer_file_contents())
    }
}
