use super::{Command, Rule};

pub struct Fab;

impl Rule for Fab {
    fn match(&self, command: &Command) -> bool {
        command.output.contains("Warning: Command(s) not found:")
    }

    fn get_new_command(&self, command: &Command) -> Vec<String> {
        let not_found_commands = get_between(
            &command.output,
            "Warning: Command(s) not found:",
            Some("Available commands:"),
        );
        let possible_commands = get_between(
            &command.output,
            "Available commands:",
            None,
        );

        let mut script = command.script.clone();
        for not_found in &not_found_commands {
            if let Some(fix) = get_closest(not_found, &possible_commands) {
                script = script.replace(
                    &