use super::{Command, Rule};

pub struct OmnienvNoSuchCommand;

impl Rule for OmnienvNoSuchCommand {
    fn name(&self) -> &str { "omnienv_no_such_command" }

    fn matches(&self, cmd: &Command) -> bool {
        cmd.output.contains("no such command")
    }

    fn fix(&self, cmd: &Command) -> String {
        cmd.text.clone()
    }
}
