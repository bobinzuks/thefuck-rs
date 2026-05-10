use super::{Command, Rule};

pub struct GemUnknownCommand;

impl Rule for GemUnknownCommand {
    fn name(&self) -> &str { "gem_unknown_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("gem") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
