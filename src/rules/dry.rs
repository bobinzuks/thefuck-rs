use super::{Command, Rule};

pub struct Dry;

impl Rule for Dry {
    fn name(&self) -> &str { "dry" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("dry") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
