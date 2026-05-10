use super::{Command, Rule};

pub struct AzCli;

impl Rule for AzCli {
    fn name(&self) -> &str { "az_cli" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("az") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
