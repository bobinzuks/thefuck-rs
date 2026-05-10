use super::{Command, Rule};

pub struct GitCloneMissing;

impl Rule for GitCloneMissing {
    fn name(&self) -> &str { "git_clone_missing" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
