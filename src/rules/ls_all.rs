use super::{Command, Rule};

pub struct LsAll;

impl Rule for LsAll {
    fn name(&self) -> &str { "ls_all" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("ls") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
