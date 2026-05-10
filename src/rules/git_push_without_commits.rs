use super::{Command, Rule};

pub struct GitPushWithoutCommits;

impl Rule for GitPushWithoutCommits {
    fn name(&self) -> &str { "git_push_without_commits" }
    fn matches(&self, cmd: &Command) -> bool { cmd.text.contains("git") }
    fn fix(&self, cmd: &Command) -> String { cmd.text.clone() }
}
