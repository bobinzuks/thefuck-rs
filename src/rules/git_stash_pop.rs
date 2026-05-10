use super::{Command, Rule};

pub struct GitStashPop;

impl Rule for GitStashPop {
    fn name(&self) -> &str { "git_stash_pop" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
