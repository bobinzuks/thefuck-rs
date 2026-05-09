use super::{Command, Rule};
use std::process::Command as ProcessCommand;

pub struct Chocolatey;

impl Rule for Chocolatey {
    fn name(&self) -> &'static str {
        "chocolatey"
    }

    fn enabled_by_default(&self) -> bool {
        ProcessCommand::new("choco")
            .output()
            .is_ok()
            || ProcessCommand::new("cinst")
                .output()
                .is_ok()
    }

    fn match(&self, command: &Command) -> bool {
        let starts_with_choco = command.script.starts_with("choco install");
        let contains_cinst = command.script_parts.iter().any(|part| part == "cinst");
        let output_contains_installing = command
            .output
            .as_ref()
            .map_or(false, |output| output.contains("Installing the following pack