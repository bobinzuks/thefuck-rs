use super::{Command, Rule};

pub struct GrepRecursive;

impl Rule for GrepRecursive {
    fn name(&self) -> &str { "grep_recursive" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("grep") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
