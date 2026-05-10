use super::{Command, Rule};

pub struct CdCs;

impl Rule for CdCs {
    fn name(&self) -> &str { "cd_cs" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("cd") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
