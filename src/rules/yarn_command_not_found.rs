use super::{Command, Rule};

pub struct YarnCommandNotFound;

impl Rule for YarnCommandNotFound {
    fn name(&self) -> &str { "yarn_command_not_found" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("yarn") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
