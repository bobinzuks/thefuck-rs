use super::{Command, Rule};

pub struct NpmWrongCommand;

impl Rule for NpmWrongCommand {
    fn name(&self) -> &str { "npm_wrong_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("npm") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
