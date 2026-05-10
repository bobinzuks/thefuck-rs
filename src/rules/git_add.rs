use super::{Command, Rule};

pub struct GitAdd;

impl Rule for GitAdd {
    fn name(&self) -> &str { "git_add" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
