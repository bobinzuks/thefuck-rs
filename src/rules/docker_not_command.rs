use super::{Command, Rule};
use crate::shell::Shell;
use std::process::Command as StdCommand;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static DOCKER_COMMANDS_CACHE: Lazy<Mutex<Option<Vec<String>>>> = Lazy::new(|| Mutex::new(None));

pub struct DockerNotCommand;

impl Rule for DockerNotCommand {
    fn name(&self) -> &'static str {
        "docker_not_command"
    }

    fn match(&self, command: &Command) -> bool {
        command.output.contains("is not a docker command")
            || command.output.contains("Usage:\tdocker")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        if command.output.contains("Usage:") && command.script_parts.len() > 1 {
            let management_subcommands = parse_commands(&command.output, "Commands:");
            retu