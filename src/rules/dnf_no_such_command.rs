use super::{Command, Rule};

pub struct DnfNoSuchCommand;

impl Rule for DnfNoSuchCommand {
    fn name(&self) -> &str { "dnf_no_such_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("dnf") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
