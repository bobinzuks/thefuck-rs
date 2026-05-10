use super::{Command, Rule};

pub struct Man;

impl Rule for Man {
    fn name(&self) -> &str { "man" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("man") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
