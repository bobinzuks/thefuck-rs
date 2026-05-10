use super::{Command, Rule};

pub struct BrewReinstall;

impl Rule for BrewReinstall {
    fn name(&self) -> &str { "brew_reinstall" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("brew") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
