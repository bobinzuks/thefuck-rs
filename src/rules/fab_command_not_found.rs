use super::{Command, Rule};

pub struct FabCommandNotFound;

impl Rule for FabCommandNotFound {
    fn name(&self) -> &str { "fab_command_not_found" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("fab") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
