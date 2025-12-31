# systemd-user-timers

`usertimers`: A command-line tool for creating systemd timers that run under the current user account.

## Purpose

`usertimers` provides an easy way to add, remove, and list *user* timers—which are systemd timers that are set up by a user, are configured in the user's home directory, and run as the user—on a Linux system. User timers can be used to kick off tasks such as backup and wallpaper changing that are executed while the user is logged in.

**Note**: System timers, which are defined in `/etc/systemd/system`, are not supported by this application.
**Note**: This app is under construction.
The following systemd commands can be used to manage user timers. Adding and removing user timers is a bit more involved, which is the primary reason this tool was written.

```bash
systemctl --user list-timers  # list user timers
systemctl --user start TIMER  # start user timer
systemctl --user stop TIMER   # stop user timer
systemctl --user status TIMER # display status of a timer
journalctl --user TIMER       # display log of a timer
```

## Installation
Currently, there is no package for this, so you need to install it manually. You can do so by using the dev container inside Visual Studio Code (needs Dev Containers extension and Docker installed). If you don't want to use the container, you have to have the Rust toolchain installed. To install the Rust toolchain, type in this:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This command will build an executable:
```bash
cargo build
```

Run:
```bash
cargo run <ARGS>
```

Run the executable without Cargo:
```bash
./target/debug/usertimers <ARGS>

```

Build as release:
```bash
cargo build --release
```

Run as release:
```bash
cargo run --release <ARGS>
```

Run as release without Cargo:
```bash
./target/release/usertimers <ARGS>
```

Install (builds as release and installs):
```bash
cargo install --path
```

Run after install:
```bash
usertimers <ARGS>
```
**Note**: <ARGS> is a placeholder for the arguments that you type in these commands.

**Note**: Our GitHub repository does not include the "target" folder, and we don't have cross-compile tools, and you can build it yourself.

## Contributing packages
View this page: https://github.com/galacticSystemsInDevelopment/systemd-user-timers-packages/

## Functionality (subcommands)

The `usertimers` command's functionality is broken out into numerous subcommands, which are listed below. Execute subcommands with the syntax `usertimers SUBCOMMAND`.

| Subcommand | Description |
|---|---|
| `add` | Add a new user timer |
| `list` | List user timers |
| `enable` | Enable an existing user timer |
| `disable` | Disable an existing user timer |
| `remove` | Remove an existing user timer |
| `status` | Display status of an existing user timer |
| `help` | Display command line usage information |

## Prerequisites

To add a user timer, you must first have something to execute. For the sake of the examples below, we assume you wish to execute a shell script you wrote named `task.sh`.

Perform the following steps to schedule a task that runs under your user account:

1. Create a script that performs a task (`~/.local/bin/task.sh`)
2. Make the script executable (`chmod +x ~/.local/bin/task.sh`)

After those first two steps, the following steps are needed, either by using `usertimers` or the equivalent systemd commands.
3. Create a `task.service` file in `~/.config/systemd/user`
4. Create a `task.timer` file in `~/.config/systemd/user`
5. Reload the systemd daemon

## How to add a new user timer

### With `usertimers`

Creating and starting a timer to execute a script named `task.sh` can be done in one command.

```bash
usertimers add --desc "Execute task" --exec ~/.local/bin/task.sh --schedule weekly --exec-if-missed
```

By default, the timer name is set to the executable name without its file extension (`task` in this example). The timer name can be set explicitly by adding a `--name <task_name>` parameter.

Note that, in this example, `task.sh` resides in the `~/.local/bin` directory, which is the default location for user-specific executables.

### Equivalent systemd commands

Creating and starting the same timer without `usertimers` takes multiple steps, as follows.

1. Create service file (`~/.config/systemd/user/task.service`) with your favorite text editor and set its contents to:

```ini
[Unit]
Description=Execute task

[Service]
Type=oneshot
ExecStart=/path/to/your/task/script.sh
```

2. Create timer file (`~/.config/systemd/user/task.timer`) with your favorite text editor and set its contents to:

```ini
[Unit]
Description=Execute task

[Timer]
OnCalendar=weekly
Persistent=true

[Install]
WantedBy=timers.target
```

3. Execute the following shell commands:

```bash
systemctl --user daemon-reload
systemctl --user enable task.timer
systemctl --user start task.timer
```

## How to list user timers

### With `usertimers`

```bash
usertimers list
```

### Equivalent systemd command

```bash
systemctl --user list-timers
```

## How to enable an existing user timer

### With `usertimers`

```bash
usertimers enable task
```

### Equivalent systemd command

```bash
systemctl --user enable --now task.timer
```

## How to disable an existing user timer

### With `usertimers`

```bash
usertimers disable task
```

### Equivalent systemd command

```bash
systemctl --user disable --now task.timer
```

## How to remove an existing user timer

### With `usertimers`

```bash
usertimers remove task
```

### Equivalent systemd commands

1. Execute the following shell command

```bash
systemctl --user disable --now task.timer
```

2. Remove the .timer and .service files

```bash
rm ~/.config/systemd/user/task.timer
rm ~/.config/systemd/user/task.service
```

3. Reload the daemon

```bash
systemctl --user daemon-reload
```

## How to display status of an existing user timer

### With `usertimers`

```bash
usertimers status task
```

### Equivalent systemd command

```bash
journalctl --user task.timer
```
