use super::{Command, Rule};

pub struct ChocoInstall;

impl Rule for ChocoInstall {
    fn name(&self) -> &str { "choco_install" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("choco") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
