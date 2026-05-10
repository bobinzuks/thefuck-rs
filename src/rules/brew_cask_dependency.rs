use super::{Command, Rule};

pub struct BrewCaskDependency;

impl Rule for BrewCaskDependency {
    fn name(&self) -> &str { "brew_cask_dependency" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("brew") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
