use super::{Command, Rule};
use std::process::Command as StdCommand;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static APT_AVAILABLE: Lazy<bool> = Lazy::new(|| {
    StdCommand::new("apt-get")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
});

static CNF_AVAILABLE: Lazy<bool> = Lazy::new(|| {
    StdCommand::new("command-not-found")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
});

fn get_executable(command: &Command) -> &str {
    if command.script_parts[0] == "sudo" {
        &command.script_parts[1]
    } else {
        &command.script_parts[0]
    }
}

fn get_package(executable: &str) -> Option<String> {
    if !*CNF_AVAILABLE {
        r