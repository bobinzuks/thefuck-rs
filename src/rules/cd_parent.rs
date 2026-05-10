use super::{Command, Rule};

pub struct CdParent;

impl Rule for CdParent {
    fn name(&self) -> &str { "cd_parent" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("cd") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
