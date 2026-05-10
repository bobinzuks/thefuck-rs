use super::{Command, Rule};

pub struct GitCommitAmend;

impl Rule for GitCommitAmend {
    fn name(&self) -> &str { "git_commit_amend" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
