use super::{Command, Rule};

pub struct AptListUpgradable;

impl Rule for AptListUpgradable {
    fn name(&self) -> &str { "apt_list_upgradable" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("apt") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
