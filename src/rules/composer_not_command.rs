use super::{Command, Rule};

pub struct ComposerNotCommand;

impl Rule for ComposerNotCommand {
    fn name(&self) -> &str { "composer_not_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("composer") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
