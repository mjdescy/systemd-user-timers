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
cargo install systemd-user-timers
```

## Subcommands
README.md will be updated to include subcommands soon.
