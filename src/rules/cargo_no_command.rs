use super::{Command, Rule};

pub struct CargoNoCommand;

impl Rule for CargoNoCommand {
    fn name(&self) -> &str { "cargo_no_command" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("cargo") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
