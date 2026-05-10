use super::{Command, Rule};

pub struct GrepArgumentsOrder;

impl Rule for GrepArgumentsOrder {
    fn name(&self) -> &str { "grep_arguments_order" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("grep") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
