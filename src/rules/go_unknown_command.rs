use super::{Command, Rule};

pub struct GoUnknownCommand;

impl Rule for GoUnknownCommand {
    fn name(&self) -> &str { "go_unknown_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("go") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
