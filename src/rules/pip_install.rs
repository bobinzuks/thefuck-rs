use super::{Command, Rule};

pub struct PipInstall;

impl Rule for PipInstall {
    fn name(&self) -> &str { "pip_install" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("pip") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
