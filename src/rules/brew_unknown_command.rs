use super::{Command, Rule};

pub struct BrewUnknownCommand;

impl Rule for BrewUnknownCommand {
    fn name(&self) -> &str { "brew_unknown_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("brew") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
