use super::{Command, Rule};

pub struct DoubleWord;

impl Rule for DoubleWord {
    fn priority(&self) -> u32 {
        900
    }

    fn match(&self, command: &Command) -> bool {
        let parts = &command.script_parts;
        parts.len() >= 2 && parts[0] == parts[1]
    }

    fn get_new_command(&self, command: &Command) -> String {
        command.script_parts[1..].join(" ")
    }
}
