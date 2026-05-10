use super::{Command, Rule};

pub struct AgLiteral;

impl Rule for AgLiteral {
    fn name(&self) -> &str { "ag_literal" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("ag") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
