use super::{Command, Rule};
use std::path::Path;

pub struct Mkdir;

impl Rule for Mkdir {
    fn name(&self) -> &'static str {
        "mkdir"
    }

    fn matches(&self, command: &Command) -> bool {
        (command.output.contains("No such file or directory")
            || (command.output.starts_with("cp: directory")
                && command.output.trim_end().ends_with("does not exist")))
            && (command.script.starts_with("cp ") || command.script.starts_with("mv "))
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let last_arg = command.script_parts.last().unwrap_or(&String::new());
        vec![format!("mkdir -p {}", last_arg), command.script.clone()]
    }
}
