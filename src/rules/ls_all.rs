use super::{Command, Rule};

pub struct LsAll;

impl Rule for LsAll {
    fn name(&self) -> &'static str {
        "ls_all"
    }

    fn matches(&self, command: &Command) -> bool {
        command.output.trim() == ""
    }

    fn fix(&self, command: &Command) -> String {
        let mut parts = vec!["ls".to_string(), "-A".to_string()];
        if command.script_parts.len() > 1 {
            parts.extend_from_slice(&command.script_parts[1..]);
        }
        parts.join(" ")
    }
}