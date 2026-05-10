use super::{Command, Rule};

pub struct BrewLink;

impl Rule for BrewLink {
    fn name(&self) -> &str { "brew_link" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("brew") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
