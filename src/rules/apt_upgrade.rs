use super::{Command, Rule};

pub struct AptUpgrade;

impl Rule for AptUpgrade {
    fn name(&self) -> &str { "apt_upgrade" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("apt") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
