use super::{Command, Rule};

pub struct Unsudo;

impl Rule for Unsudo {
    fn name(&self) -> &str { "unsudo" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("unsudo") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
