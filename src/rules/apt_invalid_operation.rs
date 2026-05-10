use super::{Command, Rule};

pub struct AptInvalidOperation;

impl Rule for AptInvalidOperation {
    fn name(&self) -> &str { "apt_invalid_operation" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("apt") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
