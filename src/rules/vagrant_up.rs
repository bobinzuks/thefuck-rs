use super::{Command, Rule};

pub struct VagrantUp;

impl Rule for VagrantUp {
    fn name(&self) -> &str { "vagrant_up" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("vagrant") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
