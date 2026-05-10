use super::{Command, Rule};

pub struct PipUnknownCommand;

impl Rule for PipUnknownCommand {
    fn name(&self) -> &str { "pip_unknown_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("pip") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
