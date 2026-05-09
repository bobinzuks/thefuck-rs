use super::{Command, Rule};

const ADB_COMMANDS: &[&str] = &[
    "backup",
    "bugreport",
    "connect",
    "devices",
    "disable-verity",
    "disconnect",
    "enable-verity",
    "emu",
    "forward",
    "get-devpath",
    "get-serialno",
    "get-state",
    "install",
    "install-multiple",
    "jdwp",
    "keygen",
    "kill-server",
    "logcat",
    "pull",
    "push",
    "reboot",
    "reconnect",
    "restore",
    "reverse",
    "root",
    "run-as",
    "shell",
    "sideload",
    "start-server",
    "sync",
    "tcpip",
    "uninstall",
    "unroot",
    "usb",
    "wait-for",
];

fn is_app(command: &Command, app: &str) -> bool {
    command.script_parts.first().map_or(false, |part| part == app)
}

fn get_closest(arg: &str, choices: &[&str]) -> &str {
    let mut bes