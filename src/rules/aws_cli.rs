use super::{Command, Rule};

pub struct AwsCli;

impl Rule for AwsCli {
    fn name(&self) -> &str { "aws_cli" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("aws") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
