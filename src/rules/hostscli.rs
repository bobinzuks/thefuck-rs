use super::{Command, Rule};

pub struct Hostscli;

impl Rule for Hostscli {
    fn name(&self) -> &str { "hostscli" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("hostscli") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
