use super::{Command, Rule};

pub struct BrewUninstall;

impl Rule for BrewUninstall {
    fn name(&self) -> &str { "brew_uninstall" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("brew") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
