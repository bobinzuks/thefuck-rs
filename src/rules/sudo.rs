use super::{Command, Rule};

pub struct Sudo;

impl Rule for Sudo {
    fn name(&self) -> &str { "sudo" }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("Permission denied") ||
        cmd.output.contains("Operation not permitted")
    }

    fn fix(&self, cmd: &Command) -> String {
        format!("sudo {}", cmd.text)
    }
}
