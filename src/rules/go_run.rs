use super::{Command, Rule};

pub struct GoRun;

impl Rule for GoRun {
    fn name(&self) -> &str { "go_run" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("go") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
