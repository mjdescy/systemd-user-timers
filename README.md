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

## Functionality (subcommands)

The `usertimers` command's functionality is broken out into numerous subcommands, which are listed below. Execute subcommands with the syntax `usertimers SUBCOMMAND`.

| Subcommand | Description |
|---|---|
| `add` | Add a new user timer |
| `list` | List user timers |
| `enable` | Enable an existing user timer |
| `disable` | Disable an existing user timer |
| `start` | Start an existing user timer |
| `stop` | Stop an existing user timer |
| `remove` | Remove an existing user timer |
| `status` | Display status of an existing user timer |
| `help` | Display command line usage information |

## Prerequisites

To add a user timer, you must first have something to execute. For the sake of the examples below, we assume you wish to execute a shell script you wrote named `task.sh`.

Perform the following steps to schedule a task that runs under your user account:

1. Create a script that performs a task (`~/.local/bin/task.sh`)
1. Make the script executable (`chmod +x ~/.local/bin/task.sh`)

After those first two steps, the following steps are needed, either by using `usertimers` or the equivalent systemd commands.

3. Create a `task.service` file in `~/.config/systemd/user`
3. Create a `task.timer` file in `~/.config/systemd/user`
3. Reload the systemd daemon

## How to add a new user timer

### With `usertimers`

Creating and starting a timer to execute a script named `task.sh` can be done in one command.

```bash
usertimers add --exec ~/.local/bin/task.sh --when weekly
```

By default, the timer name is set to the executable file name without its file extension (`task` in this examples). 

You can explicitly set the task name and description as well.

```bash
usertimers add --name task --desc "Execute task" --exec ~/.local/bin/task.sh --when weekly
```

### Equivalent systemd commands

Creating and starting the same timer without `usertimers` takes multiple steps, as follows.

1. Create service file (`~/.config/systemd/user/task.service`) with your favorite text editor and set its contents to:

```ini
[Unit]
Description=Execute task

[Service]
Type=oneshot
ExecStart=/home/user/.local/bin/task.sh
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
systemctl --user enable task.timer
```

## How to disable an existing user timer

### With `usertimers`

```bash
usertimers disable task
```

### Equivalent systemd command

```bash
systemctl --user disable task.timer
```

## How to start an existing user timer

### With `usertimers`

```bash
usertimers start task
```

### Equivalent systemd command

```bash
systemctl --user start task.timer
```

## How to stop an existing user timer

### With `usertimers`

```bash
usertimers stop task
```

### Equivalent systemd command

```bash
systemctl --user stop task.timer
```

## How to remove an existing user timer

### With `usertimers`

```bash
usertimers remove task
```

### Equivalent systemd commands

1. Execute the following shell command

```bash
systemctl --user disable task.timer
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
