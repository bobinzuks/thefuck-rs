use super::{Command, Rule};

pub struct NoCommand;

impl Rule for NoCommand {
    fn name(&self) -> &str { "no_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("no") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
