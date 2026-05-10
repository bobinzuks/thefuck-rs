use super::{Command, Rule};

pub struct BrewInstall;

impl Rule for BrewInstall {
    fn name(&self) -> &str { "brew_install" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("brew") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
