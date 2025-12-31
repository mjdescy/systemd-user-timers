Systemd User Timers
`usertimers`: A command-line tool for creating systemd timers that run under the current user account.

## Purpose

`usertimers` provides an easy way to add, remove, and list *user* timers—which are systemd timers that are set up by a user, are configured in the user's home directory, and run as the user—on a Linux system. User timers can be used to kick off tasks such as backup and wallpaper changing that are executed while the user is logged in.

**Note**: System timers, which are defined in `/etc/systemd/system`, are not supported by this application.
**Note**: This app doesn't let you edit existing timers yet

## Installation

Installation is easy! All you need is cargo.  

If you don't have cargo, all you need is curl. If you don't have that, look up how to install curl. Here is how to install cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Here is how to install systemd-user-timers:
```bash
cargo install systemd-user-t  -e, --exec <EXECUTABLE>     The executable the timer will run
  -m, --exec-if-missed        Execute immediately if missed
  -d, --desc <DESCRIPTION>    A description of the timer
  -s, --schedule <SCHEDULE>   The schedule for the timer
  -n, --name <NAME>           Optional: The name for the timer
      --recurring             Whether the timer is recurring
      --on-calendar           Use OnCalendar= (systemd calendar schedule) instead of OnActiveSec/OnUnitActiveSec
      --from-boot             Make schedule relative to system boot (OnBootSec=)
      --single-use            Whether the timer is single-use
      --enable-at-login       Enable the timer for the user at login
      --start-after-create    Start the timer immediately after creating it
      --service <SERVICE>     Specify service unit name to create/use
      --already-made-service  Assume the service already exists; do not write a service file
      --normal-service        Whether the timer activates a normal service instead of a one-shot
  -h, --helpimers
```

## Subcommands
Subcommands will never be included in README.md. To get help, use this command:
```bash
usertimers --help
```
You can also get help for a subcommand by typing:
```bash
usertimers <SUBCOMMAND> --help
```
