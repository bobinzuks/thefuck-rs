use super::{Command, Rule};

pub struct History;

impl Rule for History {
    fn name(&self) -> &str { "history" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("history") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
