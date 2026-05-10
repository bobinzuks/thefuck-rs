use super::{Command, Rule};

pub struct YumInvalidOperation;

impl Rule for YumInvalidOperation {
    fn name(&self) -> &str { "yum_invalid_operation" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("yum") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
