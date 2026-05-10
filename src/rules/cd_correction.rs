use super::{Command, Rule};

pub struct CdCorrection;

impl Rule for CdCorrection {
    fn name(&self) -> &str { "cd_correction" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("cd") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
