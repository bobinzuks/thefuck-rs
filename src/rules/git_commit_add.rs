use super::{Command, Rule};

pub struct GitCommitAdd;

impl Rule for GitCommitAdd {
    fn name(&self) -> &str { "git_commit_add" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
