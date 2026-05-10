use super::{Command, Rule};

pub struct GitNotCommand;

impl Rule for GitNotCommand {
    fn name(&self) -> &str { "git_not_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
